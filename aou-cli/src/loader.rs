use advent_of_utils::{error::AocError, Solution};
use either::Either;
use libloading::{Library, Symbol};
use std::collections::{hash_map, HashMap};
use std::iter;
use std::sync::Arc;

use crate::runner::Config;

#[repr(C)]
pub struct SolutionsContainer {
    solutions: *mut HashMap<u8, Box<dyn Solution>>,
}

pub struct LoadedSolutions {
    _lib: Arc<Library>,
    container: *mut SolutionsContainer,
}

impl IntoIterator for LoadedSolutions {
    type Item = (u8, Box<dyn Solution>);
    type IntoIter = hash_map::IntoIter<u8, Box<dyn Solution>>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            if !self.container.is_null() && !(*self.container).solutions.is_null() {
                let solutions = Box::from_raw((*self.container).solutions);
                solutions.into_iter()
            } else {
                HashMap::new().into_iter()
            }
        }
    }
}
impl Drop for LoadedSolutions {
    fn drop(&mut self) {
        unsafe {
            if let Ok(destroy) = self
                ._lib
                .get::<Symbol<unsafe extern "C" fn(*mut SolutionsContainer)>>(b"destroy_solutions")
            {
                destroy(self.container);
            }
        }
    }
}

impl LoadedSolutions {
    pub fn get(&self, day: &u8) -> Option<&dyn Solution> {
        unsafe {
            if self.container.is_null() || (*self.container).solutions.is_null() {
                None
            } else {
                (*(*self.container).solutions)
                    .get(day)
                    .map(|boxed_solution| boxed_solution.as_ref())
            }
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u8, &Box<dyn Solution>)> {
        unsafe {
            if !self.container.is_null() && !(*self.container).solutions.is_null() {
                Either::Left((*(*self.container).solutions).iter())
            } else {
                Either::Right(iter::empty())
            }
        }
    }
}

pub async fn load_year(config: &Config) -> Result<LoadedSolutions, AocError> {
    let paths = match config.loader_paths().await {
        Err(e) => return Err(AocError::LoadingFailed(e.to_string())),
        Ok(paths) => paths,
    };

    if paths.is_empty() {
        return Err(AocError::NoSolutionsFound);
    }

    let mut valid_libraries = Vec::new();
    let mut last_error = None;

    for path in paths {
        let path = path.path();
        unsafe {
            match Library::new(&path) {
                Ok(lib) => {
                    match lib.get::<Symbol<extern "C" fn() -> *mut SolutionsContainer>>(
                        b"create_solutions",
                    ) {
                        Ok(_) => {
                            valid_libraries.push((path, lib));
                        }
                        Err(error) => {
                            last_error = Some(error.to_string());
                        }
                    }
                }
                Err(error) => {
                    last_error = Some(error.to_string());
                }
            }
        }
    }

    match valid_libraries.len() {
        0 => Err(AocError::LoadingFailed(last_error.unwrap_or_else(|| {
            "No valid solution libraries found".to_string()
        }))),
        1 => {
            let (_, lib) = valid_libraries.remove(0);
            let lib = Arc::new(lib);

            unsafe {
                let create_solutions = lib
                    .get::<Symbol<extern "C" fn() -> *mut SolutionsContainer>>(b"create_solutions")
                    .map_err(|e| AocError::LoadingFailed(e.to_string()))?;

                let container = create_solutions();
                if container.is_null() {
                    return Err(AocError::LoadingFailed(
                        "Failed to create solutions container".to_string(),
                    ));
                }

                Ok(LoadedSolutions {
                    _lib: lib,
                    container,
                })
            }
        }
        _ => Err(AocError::AmbiguousSolutions(
            valid_libraries.into_iter().map(|(p, _)| p).collect(),
        )),
    }
}
