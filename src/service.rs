use std::collections::HashMap;

use crate::aws_regional_product_services::AwsRegionalProductServices;

pub trait Service {
    type Param;
    type Result;
    fn run(&self, data: &AwsRegionalProductServices, param: &Self::Param) -> Self::Result;
}

#[derive(Debug)]
pub struct ListRegion;

impl Service for ListRegion {
    type Param = ();
    type Result = Vec<String>;
    fn run(&self, data: &AwsRegionalProductServices, _param: &Self::Param) -> Self::Result {
        let regions = data
            .prices
            .iter()
            .map(|p| p.attributes.aws_region.clone())
            .collect::<std::collections::HashSet<_>>();
        let mut regions = regions.into_iter().collect::<Vec<_>>();
        regions.sort();
        regions
    }
}

#[derive(Debug)]
pub struct ListService;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListServiceParams {
    pub region: String,
}

impl ListServiceParams {
    pub fn new(region: String) -> Self {
        Self { region }
    }
}

impl Service for ListService {
    type Param = ListServiceParams;
    type Result = Vec<String>;

    fn run(&self, data: &AwsRegionalProductServices, param: &Self::Param) -> Self::Result {
        let mut services = data
            .prices
            .iter()
            .filter(|p| p.attributes.aws_region == param.region)
            .map(|p| p.attributes.aws_service_name.clone())
            .collect::<Vec<_>>();
        services.sort();
        services
    }
}

#[derive(Debug)]
pub struct ListAllService;

impl Service for ListAllService {
    type Param = ();
    type Result = HashMap<String, Vec<String>>;

    fn run(&self, data: &AwsRegionalProductServices, _param: &Self::Param) -> Self::Result {
        let mut services = HashMap::new();
        for price in &data.prices {
            let region = price.attributes.aws_region.clone();
            let service = price.attributes.aws_service_name.clone();
            services
                .entry(region)
                .or_insert_with(Vec::new)
                .push(service);
        }

        services.iter_mut().for_each(|(_, v)| v.sort());
        services
    }
}

#[derive(Debug)]
pub struct ExistsRegion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExistsRegionParams {
    pub region: String,
}

impl ExistsRegionParams {
    pub fn new(region: String) -> Self {
        Self { region }
    }
}

impl Service for ExistsRegion {
    type Param = ExistsRegionParams;
    type Result = bool;

    fn run(&self, data: &AwsRegionalProductServices, param: &Self::Param) -> Self::Result {
        let regions = ListRegion.run(data, &());
        regions.contains(&param.region)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = r#"{
    "metadata": {
      "copyright": "Copyright",
      "disclaimer": "Disclaimer",
      "format:version": "1.0",
      "source:version": "20110101123400"
    },
    "prices": [
      {
        "attributes": {
          "aws:region": "us-east-1",
          "aws:serviceName": "Amazon Elastic Compute Cloud (EC2)",
          "aws:serviceUrl": "https://aws.amazon.com/ec2/"
        },
        "id": "ec2:us-east-1"
      }
    ]
    }"#;

    #[test]
    fn test_list_region() {
        let data = serde_json::from_str::<AwsRegionalProductServices>(DATA).unwrap();
        let regions = ListRegion.run(&data, &());
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0], "us-east-1");
    }

    #[test]
    fn test_list_service() {
        let data = serde_json::from_str::<AwsRegionalProductServices>(DATA).unwrap();
        let services = ListService.run(&data, &ListServiceParams::new("us-east-1".to_string()));
        assert_eq!(services.len(), 1);
        assert_eq!(services[0], "Amazon Elastic Compute Cloud (EC2)");
    }

    #[test]
    fn test_list_all_service() {
        let data = serde_json::from_str::<AwsRegionalProductServices>(DATA).unwrap();
        let services = ListAllService.run(&data, &());
        assert_eq!(services.len(), 1);
        assert_eq!(services.get("us-east-1").unwrap().len(), 1);
        assert_eq!(
            services.get("us-east-1").unwrap()[0],
            "Amazon Elastic Compute Cloud (EC2)"
        );
    }

    #[test]
    fn test_exists_region() {
        let data = serde_json::from_str::<AwsRegionalProductServices>(DATA).unwrap();
        assert!(ExistsRegion.run(&data, &ExistsRegionParams::new("us-east-1".to_string())));
        assert!(!ExistsRegion.run(&data, &ExistsRegionParams::new("us-east-2".to_string())));
    }
}
