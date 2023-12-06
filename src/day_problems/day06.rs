// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
// };
// use axum::extract::Json;

#[allow(dead_code)]
fn find_elfs(s: &str, count: &mut i32) -> i32 {
    if let Some(ind) = s.find("elf") {
        *count += 1;
        find_elfs(&s[ind + 3..], count) // offset an "elf"
    } else {
        *count
    }
}

#[allow(dead_code)]
fn find_elfs_on_shelves(s: &str, count: &mut i32) -> i32 {
    if let Some(ind) = s.find("elf on a shelf") {
        *count += 1;
        find_elfs_on_shelves(&s[ind + 11..], count) // offset an "elf on a sh"
    } else {
        *count
    }
}

#[allow(dead_code)]
fn find_shelves_with_no_elfs(s: &str, count: &mut i32) -> i32 {
    if let Some(ind) = s.find("shelf") {
        if ind <= 8 || s[ind - 9..ind + 5] != *"elf on a shelf" {
            *count += 1;
        }
        // // if ind > 8 {
        // //     println!("{}", &s[ind - 9..ind + 5]);
        // // }
        // println!("{}", &s[ind + 2..]);
        find_shelves_with_no_elfs(&s[ind + 2..], count) // offset a "sh"
    } else {
        *count
    }
}

#[cfg(test)]
mod tests {
    use super::find_elfs;
    use super::find_elfs_on_shelves;
    use super::find_shelves_with_no_elfs;

    #[test]
    fn test_find_elfs() {
        let mut count = 0;
        find_elfs(
            r"The mischievous elf peeked out from behind the toy workshop,
      and another elf joined in the festive dance.
      Look, there is also an elf on that shelf!",
            &mut count,
        );
        assert_eq!(count, 4);

        count = 0;
        find_elfs("elfelfelf", &mut count);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_find_elfs_on_shelves() {
        let mut count = 0;
        find_elfs_on_shelves(
            r"there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.",
            &mut count,
        );
        assert_eq!(count, 1);

        count = 0;
        find_elfs_on_shelves("elf on a shelf on a shelf", &mut count);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_find_shelves_with_no_elfs() {
        let mut count = 0;
        find_elfs_on_shelves(
            r"there is an elf on a shelf on an elf.
        there is also another shelf in Belfast.",
            &mut count,
        );
        assert_eq!(count, 1);

        count = 0;
        find_shelves_with_no_elfs("shelf", &mut count);
        assert_eq!(count, 1);

        count = 0;
        find_shelves_with_no_elfs("elf on a shelf", &mut count);
        assert_eq!(count, 0);

        count = 0;
        find_shelves_with_no_elfs("shelf on a shelf", &mut count);
        assert_eq!(count, 1);
    }
}
