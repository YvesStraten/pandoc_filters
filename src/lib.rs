use std::ops::{Deref, DerefMut};

use pandoc_types::definition::{Block, Inline, IterBlocks, IterInlines};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedPandoc(pub pandoc_types::definition::Pandoc);

impl Deref for ModifiedPandoc {
    type Target = pandoc_types::definition::Pandoc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModifiedPandoc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait PandocFilter<I> {
    fn apply(&mut self, item: &mut I);
}

pub trait Filterer<I> {
    fn add_filter<F: PandocFilter<I>>(&mut self, filter: F);

    fn add_filters<F: PandocFilter<I>>(&mut self, filters: Vec<F>);
}

impl Filterer<Inline> for ModifiedPandoc {
    fn add_filter<F: PandocFilter<Inline>>(&mut self, mut filter: F) {
        for block in self.iter_blocks_mut() {
            for inline in block.iter_inlines_mut() {
                filter.apply(inline);
            }
        }
    }

    fn add_filters<F: PandocFilter<Inline>>(&mut self, filters: Vec<F>) {
        for filter in filters {
            Self::add_filter(self, filter);
        }
    }
}

impl Filterer<Block> for ModifiedPandoc {
    fn add_filter<F: PandocFilter<Block>>(&mut self, mut filter: F) {
        for block in self.iter_blocks_mut() {
            filter.apply(block)
        }
    }

    fn add_filters<F: PandocFilter<Block>>(&mut self, filters: Vec<F>) {
        for filter in filters {
            Self::add_filter(self, filter);
        }
    }
}

pub type Pandoc = ModifiedPandoc;
