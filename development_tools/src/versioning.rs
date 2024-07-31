use semver::{Error, Version, VersionReq};

/// Find the maximum version that satisfies the given version requirement string.
///
/// # Arguments
///
/// * `version_req_str` - A string slice that holds the version requirement.
/// * `iterable` - An iterator over a collection of strings that can be parsed into versions.
///
/// # Returns
///
/// * `Ok(Some(Version))` - The maximum version that satisfies the version requirement.
/// * `Ok(None)` - If no version satisfies the version requirement.
/// * `Err(Error)` - If the version requirement string is invalid.
///
/// # Example
///
/// ```
/// use development_tools::find_max_matching_version;
/// use semver::Version;
///
/// assert_eq!(
///    find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"]).unwrap(),
///    Some(Version::parse("1.0.0").unwrap())
/// );
/// ```
pub fn find_max_matching_version<'a, I>(
    version_req_str: &str,
    iterable: I,
) -> Result<Option<Version>, Error>
where
    I: IntoIterator<Item = &'a str>,
{
    let v_req = VersionReq::parse(version_req_str)?;

    Ok(iterable
        .into_iter()
        .filter_map(|s| Version::parse(s).ok())
        .filter(|s| v_req.matches(s))
        .max())
}
