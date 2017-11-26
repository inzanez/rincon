
extern crate tokio_core;

extern crate arangodb_core;
extern crate arangodb_connector;
extern crate arangodb_client;
extern crate arangodb_test_helper;

use arangodb_test_helper::*;
use arangodb_client::admin::*;

#[test]
fn get_target_version() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetTargetVersion::new();
        let work = conn.execute(method);
        let target_version = core.run(work).unwrap();

        assert_eq!("30208", target_version.version());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_without_details() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("arango", server_version.server());
        assert_eq!("community", server_version.license());
        assert_eq!("3.2.8", server_version.version());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_major_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("3", server_version.major());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_minor_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("2", server_version.minor());

    }, |_, _| {
    });
}

#[test]
fn get_server_version_sub_part() {
    arango_system_db_test(|conn, ref mut core| {

        let method = GetServerVersion::new();
        let work = conn.execute(method);
        let server_version = core.run(work).unwrap();

        assert_eq!("8", server_version.sub());

    }, |_, _| {
    });
}