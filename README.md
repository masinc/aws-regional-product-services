# aws-regional-product-services

# Overview

This project provides a command-line interface to interact with AWS Regional Product Services. The project is written in
Rust.

# Features

* List all AWS services in a region
* List all AWS regions
* Diff AWS services between regions

# Usage

The project offers a command-line interface for interacting with AWS Regional Product Services.

## List all AWS services in a region

```bash
aws-regional-product-services list ap-nottheast-1
```

<details>
<summary>Output</summary>

```bash
AWS Amplify
AWS App Mesh
AWS App Runner
AWS AppSync
AWS Application Discovery Service
AWS Application Migration Service (MGN)
AWS Artifact
AWS Audit Manager
AWS Auto Scaling
AWS Backup
AWS Batch
AWS Budgets
AWS Certificate Manager
AWS Chatbot
AWS Clean Rooms
AWS Cloud Control API
AWS Cloud Map
AWS Cloud9
AWS CloudFormation
AWS CloudHSM
AWS CloudShell
AWS CloudTrail
AWS CodeArtifact
AWS CodeBuild
AWS CodeCommit
AWS CodeDeploy
AWS CodePipeline
AWS CodeStar
AWS Compute Optimizer
AWS Config
AWS Control Tower
AWS Data Exchange
AWS Data Pipeline
AWS DataSync
AWS Database Migration Service
AWS DeepLens
AWS Direct Connect
AWS Directory Service
AWS Elastic Beanstalk
AWS Elastic Disaster Recovery (DRS)
AWS Elemental MediaConnect
AWS Elemental MediaConvert
AWS Elemental MediaLive
AWS Elemental MediaPackage
AWS Elemental MediaStore
AWS Elemental MediaTailor
AWS Fargate
AWS Fault Injection Simulator
AWS Firewall Manager
AWS Global Accelerator
AWS Glue
AWS Health Dashboard
AWS IAM Identity Center
AWS IQ
AWS Identity and Access Management (IAM)
AWS IoT 1-Click
AWS IoT Analytics
AWS IoT Core
AWS IoT Device Defender
AWS IoT Device Management
AWS IoT Events
AWS IoT Greengrass
AWS IoT SiteWise
AWS Key Management Service
AWS Lake Formation
AWS Lambda
AWS Launch Wizard
AWS License Manager
AWS Mainframe Modernization
AWS Managed Services
AWS Marketplace
AWS Migration Hub
AWS Network Firewall
AWS OpsWorks Stacks
AWS OpsWorks for Chef Automate
AWS OpsWorks for Puppet Enterprise
AWS Organizations
AWS Outposts
AWS Private Certificate Authority
AWS PrivateLink
AWS Proton
AWS Resilience Hub
AWS Resource Access Manager (RAM)
AWS Resource Groups
AWS RoboMaker
AWS Secrets Manager
AWS Security Hub
AWS Serverless Application Repository
AWS Service Catalog
AWS Shield
AWS Signer
AWS Snowball
AWS Snowcone
AWS Step Functions
AWS Storage Gateway
AWS Support
AWS Systems Manager
AWS Transfer Family
AWS Transit Gateway
AWS Trusted Advisor
AWS User Notifications
AWS VPN
AWS WAF
AWS Well-Architected Tool
AWS X-Ray
Amazon API Gateway
Amazon AppFlow
Amazon AppStream 2.0
Amazon Athena
```

</details>

## Diff AWS services between regions

```bash
aws-regional-product-services diff ap-nottheast-1 ap-southeast-3
```

<details>
<summary>Output</summary>

```bash
- AWS Amplify
  AWS App Mesh
- AWS App Runner
  AWS AppSync
- AWS Application Discovery Service
  AWS Application Migration Service (MGN)
  AWS Artifact
- AWS Audit Manager
  AWS Auto Scaling
  AWS Backup
  AWS Batch
- AWS Budgets
  AWS Certificate Manager
  AWS Chatbot
- AWS Clean Rooms
  AWS Cloud Control API
  AWS Cloud Map
  AWS Cloud9
  AWS CloudFormation
  AWS CloudHSM
  AWS CloudShell
  AWS CloudTrail
- AWS CodeArtifact
  AWS CodeBuild
  AWS CodeCommit
  AWS CodeDeploy
- AWS CodePipeline
- AWS CodeStar
  AWS Compute Optimizer
  AWS Config
  AWS Control Tower
- AWS Data Exchange
- AWS Data Pipeline
  AWS DataSync
  AWS Database Migration Service
- AWS DeepLens
  AWS Direct Connect
  AWS Directory Service
  AWS Elastic Beanstalk
  AWS Elastic Disaster Recovery (DRS)
  AWS Elemental MediaConnect
  AWS Elemental MediaConvert
  AWS Elemental MediaLive
- AWS Elemental MediaPackage
- AWS Elemental MediaStore
- AWS Elemental MediaTailor
  AWS Fargate
- AWS Fault Injection Simulator
  AWS Firewall Manager
  AWS Global Accelerator
  AWS Glue
  AWS Health Dashboard
  AWS IAM Identity Center
  AWS IQ
  AWS Identity and Access Management (IAM)
- AWS IoT 1-Click
- AWS IoT Analytics
- AWS IoT Core
- AWS IoT Device Defender
- AWS IoT Device Management
- AWS IoT Events
- AWS IoT Greengrass
- AWS IoT SiteWise
  AWS Key Management Service
  AWS Lake Formation
  AWS Lambda
  AWS Launch Wizard
  AWS License Manager
- AWS Mainframe Modernization
- AWS Managed Services
  AWS Marketplace
- AWS Migration Hub
  AWS Network Firewall
- AWS OpsWorks Stacks
- AWS OpsWorks for Chef Automate
- AWS OpsWorks for Puppet Enterprise
  AWS Organizations
  AWS Outposts
  AWS Private Certificate Authority
  AWS PrivateLink
- AWS Proton
- AWS Resilience Hub
  AWS Resource Access Manager (RAM)
  AWS Resource Groups
- AWS RoboMaker
  AWS Secrets Manager
  AWS Security Hub
- AWS Serverless Application Repository
  AWS Service Catalog
  AWS Shield
- AWS Signer
  AWS Snowball
- AWS Snowcone
  AWS Step Functions
  AWS Storage Gateway
  AWS Support
  AWS Systems Manager
  AWS Transfer Family
  AWS Transit Gateway
  AWS Trusted Advisor
  AWS User Notifications
  AWS VPN
  AWS WAF
- AWS Well-Architected Tool
  AWS X-Ray
  Amazon API Gateway
- Amazon AppFlow
- Amazon AppStream 2.0
  Amazon Athena
- Amazon Augmented AI (A2I)
  Amazon Aurora
- Amazon Chime
- Amazon Chime SDK
  Amazon CloudFront
- Amazon CloudSearch
  Amazon CloudWatch
  Amazon CloudWatch Logs
- Amazon CodeGuru
- Amazon Cognito
- Amazon Comprehend
- Amazon Connect
- Amazon Detective
- Amazon DevOps Guru
- Amazon DocumentDB (with MongoDB compatibility)
  Amazon DynamoDB
  Amazon EC2 Auto Scaling
  Amazon ElastiCache
  Amazon Elastic Block Store (EBS)
  Amazon Elastic Compute Cloud (EC2)
  Amazon Elastic Container Registry (ECR)
  Amazon Elastic Container Service (ECS)
  Amazon Elastic File System (EFS)
- Amazon Elastic Inference
  Amazon Elastic Kubernetes Service (EKS)
  Amazon Elastic MapReduce (EMR)
- Amazon Elastic Transcoder
  Amazon EventBridge
  Amazon FSx
  Amazon FSx for Lustre
- Amazon FSx for NetApp ONTAP
- Amazon FSx for OpenZFS
  Amazon FSx for Windows File Server
- Amazon File Cache
- Amazon Forecast
  Amazon GameLift
  Amazon GuardDuty
- Amazon IVS
- Amazon Inspector
- Amazon Inspector Classic
- Amazon Kendra
- Amazon Keyspaces (for Apache Cassandra)
  Amazon Kinesis Data Analytics
  Amazon Kinesis Data Firehose
  Amazon Kinesis Data Streams
- Amazon Kinesis Video Streams
- Amazon Lex
- Amazon Lightsail
- Amazon Location Service
- Amazon Lookout for Metrics
- Amazon Lookout for Vision
- Amazon Lumberyard
  Amazon MQ
  Amazon Macie
- Amazon Managed Blockchain
- Amazon Managed Grafana
- Amazon Managed Service for Prometheus
  Amazon Managed Streaming for Apache Kafka
- Amazon Managed Workflows for Apache Airflow
- Amazon MemoryDB for Redis
- Amazon Neptune
- Amazon Nimble Studio
  Amazon OpenSearch Service
- Amazon Personalize
- Amazon Pinpoint
  Amazon Polly
- Amazon Quantum Ledger Database (QLDB)
- Amazon QuickSight
  Amazon Redshift
- Amazon Rekognition
  Amazon Relational Database Service (RDS)
  Amazon Route 53
  Amazon Route 53 Application Recovery Controller
  Amazon SageMaker
- Amazon Security Lake
  Amazon Simple Email Service (SES)
  Amazon Simple Notification Service (SNS)
  Amazon Simple Queue Service (SQS)
  Amazon Simple Storage Service (S3)
  Amazon Simple Workflow Service (SWF)
- Amazon SimpleDB
- Amazon Timestream
- Amazon Transcribe
- Amazon Translate
- Amazon VPC Lattice
  Amazon Verified Permissions
  Amazon Virtual Private Cloud (VPC)
- Amazon WorkDocs
- Amazon WorkSpaces
- Amazon WorkSpaces Web
  CloudEndure Disaster Recovery
  CloudEndure Migration
  EC2 Image Builder
  Elastic Load Balancing
- FreeRTOS
  Red Hat OpenShift Service on AWS (ROSA)
  VMware Cloud on AWS
```

</details>

## List all AWS regions

```bash
aws-regional-product-services regions
```

<details>
<summary>Output</summary>

```bash
af-south-1
ap-east-1
ap-northeast-1
ap-northeast-2
ap-northeast-3
ap-south-1
ap-south-2
ap-southeast-1
ap-southeast-2
ap-southeast-3
ap-southeast-4
ca-central-1
cn-north-1
cn-northwest-1
eu-central-1
eu-central-2
eu-north-1
eu-south-1
eu-south-2
eu-west-1
eu-west-2
eu-west-3
il-central-1
me-central-1
me-south-1
sa-east-1
us-east-1
us-east-2
us-gov-east-1
us-gov-west-1
us-west-1
us-west-2
```

</details>

## Update the AWS services cache

```bash
aws-regional-product-services fetch
```

And See the CLI help for more details:

```bash
aws-regional-product-services --help
```

# License

The project is licensed under the Apache License Version 2.0 or MIT License. See the LICENSE file for more details.
