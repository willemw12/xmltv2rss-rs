//! Common export module.

use xmltv::{Programme, Tv};

pub mod rss;

use crate::xmltv::Error;

/// XMLTV export trait.
pub trait Visitor {
    type Output;

    // fn visit_tv_start(&mut self) -> Result<()> {
    //     Ok(())
    // }

    #[allow(unused)]
    fn visit_tv(&mut self, listing: &Tv) -> Result<(), Error> {
        Ok(())
    }

    // fn visit_tv_end(&mut self) -> Result<()> {
    //     Ok(())
    // }

    //

    // fn visit_channels_start(&mut self) -> Result<()> {
    //     Ok(())
    // }
    //
    // #[allow(unused)]
    // fn visit_channel(&mut self, channel: &Channel) -> Result<()> {
    //     Ok(())
    // }
    //
    // fn visit_channels_end(&mut self) -> Result<()> {
    //     Ok(())
    // }

    //

    fn visit_programmes_start(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[allow(unused)]
    fn visit_programme(&mut self, programme: &Programme) -> Result<(), Error> {
        Ok(())
    }

    fn visit_programmes_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    //

    fn result(&self) -> Result<Self::Output, Error>;
}

/// Exports from XMLTV to some type T.
pub(crate) fn export<T>(visitor: &mut impl Visitor<Output = T>, listing: &Tv) -> Result<T, Error> {
    // visitor.visit_tv_start()?;

    visitor.visit_tv(listing)?;

    // visitor.visit_channels_start()?;
    // for channel in &listing.channels {
    //     visitor.visit_channel(channel)?;
    // }
    // visitor.visit_channels_end()?;

    visitor.visit_programmes_start()?;
    for programme in &listing.programmes {
        visitor.visit_programme(programme)?;
    }
    visitor.visit_programmes_end()?;

    // visitor.visit_tv_end()?;

    visitor.result()
}
