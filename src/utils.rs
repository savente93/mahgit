use std::path::PathBuf;

/// calculates the path to the object in the git database.
/// object should be a string containing a hash
/// for a hash abcdefg the path should be .git/objects/ab/cdefg
pub fn path_from_object_name(object: String) -> PathBuf {
    let dir = &object[0..2];
    let file_name = &object[2..];
    let path = PathBuf::new()
        .join(".git")
        .join("objects")
        .join(dir)
        .join(file_name);

    path
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8() -> Result<()> {
        Ok(())
    }
    #[test]
    fn test_8386b2faa5c220a8ae74a9b5b93ab58e820c21b9() -> Result<()> {
        let test_hash = String::from("8386b2faa5c220a8ae74a9b5b93ab58e820c21b9");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/83/86b2faa5c220a8ae74a9b5b93ab58e820c21b9"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_221efa35a4b4c697c973bdbc78558d565a232dc8() -> Result<()> {
        let test_hash = String::from("221efa35a4b4c697c973bdbc78558d565a232dc8");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/22/1efa35a4b4c697c973bdbc78558d565a232dc8"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_deefedc393105f89a5ec0e9761d6da11f4c83248() -> Result<()> {
        let test_hash = String::from("deefedc393105f89a5ec0e9761d6da11f4c83248");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/de/efedc393105f89a5ec0e9761d6da11f4c83248"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_12a12f01f7ac40f557138e2e2783acecfe0518a7() -> Result<()> {
        let test_hash = String::from("12a12f01f7ac40f557138e2e2783acecfe0518a7");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/12/a12f01f7ac40f557138e2e2783acecfe0518a7"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_18f6bbc42747ff6e55bdc600f69f05865a87f20e() -> Result<()> {
        let test_hash = String::from("18f6bbc42747ff6e55bdc600f69f05865a87f20e");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/18/f6bbc42747ff6e55bdc600f69f05865a87f20e"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_d870f8fdd408cb90d045a96965f8f8d55d6ff1a3() -> Result<()> {
        let test_hash = String::from("d870f8fdd408cb90d045a96965f8f8d55d6ff1a3");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/d8/70f8fdd408cb90d045a96965f8f8d55d6ff1a3"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_9818eb6e859ccb52e72243ef29348da61cdf9db5() -> Result<()> {
        let test_hash = String::from("9818eb6e859ccb52e72243ef29348da61cdf9db5");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/98/18eb6e859ccb52e72243ef29348da61cdf9db5"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_c9dd251fedec7d835a1cf46ecfccda3d0faac937() -> Result<()> {
        let test_hash = String::from("c9dd251fedec7d835a1cf46ecfccda3d0faac937");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/c9/dd251fedec7d835a1cf46ecfccda3d0faac937"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_6257e894be730e48730978cc51ef235b469a3f15() -> Result<()> {
        let test_hash = String::from("6257e894be730e48730978cc51ef235b469a3f15");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/62/57e894be730e48730978cc51ef235b469a3f15"),
            path,
        );
        Ok(())
    }
    #[test]
    fn test_065bf26b1d29cd4c77f7eaef9ec6ed8d4501cd54() -> Result<()> {
        let test_hash = String::from("065bf26b1d29cd4c77f7eaef9ec6ed8d4501cd54");
        let path = path_from_object_name(test_hash);
        assert_eq!(
            PathBuf::from(".git/objects/06/5bf26b1d29cd4c77f7eaef9ec6ed8d4501cd54"),
            path,
        );
        Ok(())
    }
}
