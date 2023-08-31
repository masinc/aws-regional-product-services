use std::io::prelude::*;
use std::process::ExitCode;

use similar::{ChangeTag, TextDiff};
use termcolor::WriteColor;

use crate::aws_regional_product_services::{RetrieveMode, Retriever};
use crate::cli;
use crate::output::{ListAllServiceOutput, ListServiceOutput, Output};
use crate::service::{
    ExistsRegion, ExistsRegionParams, ListAllService, ListRegion, ListService, ListServiceParams,
    Service,
};

#[async_trait::async_trait]
pub trait Command {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode>;
}

#[async_trait::async_trait]
impl Command for cli::SubCommand {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode> {
        match self {
            cli::SubCommand::Region(cmd) => cmd.execute(retriever).await,
            cli::SubCommand::Fetch(cmd) => cmd.execute(retriever).await,
            cli::SubCommand::Service(cmd) => cmd.execute(retriever).await,
            cli::SubCommand::Diff(cmd) => cmd.execute(retriever).await,
        }
    }
}

#[async_trait::async_trait]
impl Command for cli::Region {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode> {
        let data = retriever.retrieve().await?;
        let regions = ListRegion.run(&data, &());

        for region in regions {
            println!("{}", region);
        }

        Ok(ExitCode::SUCCESS)
    }
}

#[async_trait::async_trait]
impl Command for cli::Fetch {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode> {
        retriever.retrieve_with(RetrieveMode::Fetch).await?;
        Ok(ExitCode::SUCCESS)
    }
}

#[async_trait::async_trait]
impl Command for cli::Service {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode> {
        let data = retriever.retrieve().await?;

        if let Some(region) = &self.region {
            // List services of the region
            if !ExistsRegion.run(&data, &ExistsRegionParams::new(region.clone())) {
                eprintln!("Region {} not found", region);
                return Ok(ExitCode::FAILURE);
            }

            let services = ListService.run(&data, &ListServiceParams::new(region.clone()));

            ListServiceOutput.write(
                &services,
                &mut std::io::stdout(),
                self.output.unwrap_or_default(),
            )?;
        } else {
            // List services of all regions
            let services = ListAllService.run(&data, &());

            ListAllServiceOutput.write(
                &services,
                &mut std::io::stdout(),
                self.output.unwrap_or_default(),
            )?;
        }

        Ok(ExitCode::SUCCESS)
    }
}

#[async_trait::async_trait]
impl Command for cli::Diff {
    async fn execute(&self, retriever: &Retriever) -> anyhow::Result<ExitCode> {
        let data = retriever.retrieve().await?;

        if !ExistsRegion.run(&data, &ExistsRegionParams::new(self.region1.clone())) {
            eprintln!("Region {} not found", self.region1);
            return Ok(ExitCode::FAILURE);
        }

        if !ExistsRegion.run(&data, &ExistsRegionParams::new(self.region2.clone())) {
            eprintln!("Region {} not found", self.region2);
            return Ok(ExitCode::FAILURE);
        }

        let service1 = ListService.run(&data, &ListServiceParams::new(self.region1.clone()));
        let service2 = ListService.run(&data, &ListServiceParams::new(self.region2.clone()));

        let service1 = service1.join("\n");
        let service2 = service2.join("\n");

        let diff = TextDiff::from_lines(&service1, &service2);

        let color_choice = if self.no_color {
            termcolor::ColorChoice::Never
        } else {
            termcolor::ColorChoice::Always
        };

        let mut stdout = termcolor::StandardStream::stdout(color_choice);

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    stdout.set_color(
                        termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)),
                    )?;
                    write!(&mut stdout, "- {}", change.value())?;
                    stdout.reset()?;
                }
                ChangeTag::Insert => {
                    stdout.set_color(
                        termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Green)),
                    )?;
                    write!(&mut stdout, "+ {}", change.value())?;
                    stdout.reset()?;
                }
                ChangeTag::Equal => {
                    if self.diff_only {
                        continue;
                    }
                    write!(&mut stdout, "  {}", change.value())?;
                }
            }
        }

        Ok(ExitCode::SUCCESS)
    }
}
