use std::io::prelude::*;

use crate::cli::OutputFormat;
use crate::service;

pub trait Output {
    type Param;
    fn write<W: Write>(
        &self,
        param: &Self::Param,
        writer: &mut W,
        format: OutputFormat,
    ) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct ListRegionOutput;

impl Output for ListRegionOutput {
    type Param = service::ListRegionResult;
    fn write<W: Write>(
        &self,
        param: &Self::Param,
        writer: &mut W,
        format: OutputFormat,
    ) -> anyhow::Result<()> {
        match format {
            OutputFormat::Text => {
                for region in param {
                    writeln!(writer, "{}", region)?;
                }
            }
            OutputFormat::Yaml => {
                serde_yaml::to_writer(writer, param)?;
            }
            OutputFormat::Json => {
                serde_json::to_writer_pretty(writer, param)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ListServiceOutput;

impl Output for ListServiceOutput {
    type Param = service::ListServiceResult;
    fn write<W: Write>(
        &self,
        param: &Self::Param,
        writer: &mut W,
        format: OutputFormat,
    ) -> anyhow::Result<()> {
        match format {
            OutputFormat::Text => {
                for service in param {
                    writeln!(writer, "{}", service)?;
                }
            }
            OutputFormat::Yaml => {
                serde_yaml::to_writer(writer, param)?;
            }
            OutputFormat::Json => {
                serde_json::to_writer_pretty(writer, param)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ListAllServiceOutput;

impl Output for ListAllServiceOutput {
    type Param = service::ListAllServiceResult;
    fn write<W: Write>(
        &self,
        param: &Self::Param,
        writer: &mut W,
        format: OutputFormat,
    ) -> anyhow::Result<()> {
        match format {
            OutputFormat::Text => {
                for (region, services) in param {
                    writeln!(writer, "{}", region)?;
                    for service in services {
                        writeln!(writer, "  {}", service)?;
                    }
                }
            }
            OutputFormat::Yaml => {
                serde_yaml::to_writer(writer, param)?;
            }
            OutputFormat::Json => {
                serde_json::to_writer_pretty(writer, param)?;
            }
        }
        Ok(())
    }
}
