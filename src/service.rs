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
