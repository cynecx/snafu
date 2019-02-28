extern crate snafu;

// There are also happy-path tests

mod outer {
    pub mod inner {
        use snafu::Snafu;

        #[derive(Debug, Snafu)]
        #[snafu_visibility = "pub(crate)"]
        pub(crate) enum Error {
            PubCrate,
            #[snafu_visibility = "pub(in ::outer)"]
            PubInPath,
            #[snafu_visibility]
            Private,
        }
    }

    fn private_is_applied() {
        let _ = self::inner::Private.fail::<()>();
        //~^ ERROR `Private` is private
    }
}

fn pub_in_path_is_applied() {
    let _ = self::outer::inner::PubInPath.fail::<()>();
    //~^ ERROR `PubInPath` is private
}

fn private_is_applied() {
    let _ = self::outer::inner::Private.fail::<()>();
    //~^ ERROR `Private` is private
}
