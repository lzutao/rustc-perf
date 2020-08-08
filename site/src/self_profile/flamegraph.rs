use analyzeme::{collapse_stacks, ProfilingData};
use anyhow::Context;
use inferno::flamegraph::{from_lines, Options as FlamegraphOptions};

#[derive(serde::Deserialize, Debug)]
pub struct Opt {}

pub fn generate(pieces: super::Pieces, _: Opt) -> anyhow::Result<Vec<u8>> {
    let profiling_data =
        ProfilingData::from_buffers(pieces.string_data, pieces.string_index, pieces.events)
            .map_err(|e| anyhow::format_err!("{:?}", e))?;

    let recorded_stacks = collapse_stacks(&profiling_data)
        .iter()
        .map(|(unique_stack, count)| format!("{} {}", unique_stack, count))
        .collect::<Vec<_>>();

    let mut file = Vec::new();
    let mut flamegraph_options = FlamegraphOptions::default();

    from_lines(
        &mut flamegraph_options,
        recorded_stacks.iter().map(|s| s.as_ref()),
        &mut file,
    )
    .context("unable to generate a flamegraph from the collapsed stack data")?;
    Ok(file)
}
