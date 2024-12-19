use core::str;
use fcsd::Set;

use crate::error::ZarrDfError;

pub struct Dictionary {
    subjects: Set,
    predicates: Set,
    objects: Set,
}

impl Dictionary {
    pub(crate) fn new(
        subjects: impl IntoIterator<Item = String>,
        predicates: impl IntoIterator<Item = String>,
        objects: impl IntoIterator<Item = String>,
    ) -> Result<Self, ZarrDfError> {
        let mut subjects: Vec<_> = subjects.into_iter().collect();
        let mut predicates: Vec<_> = predicates.into_iter().collect();
        let mut objects: Vec<_> = objects.into_iter().collect();

        subjects.sort();
        predicates.sort();
        objects.sort();

        let dictionary = Dictionary {
            subjects: Set::new(subjects)?,
            predicates: Set::new(predicates)?,
            objects: Set::new(objects)?,
        };

        Ok(dictionary)
    }

    pub fn subjects(&self) -> Set {
        self.subjects.clone()
    }

    pub fn predicates(&self) -> Set {
        self.predicates.clone()
    }

    pub fn objects(&self) -> Set {
        self.objects.clone()
    }

    pub fn get_subject_idx(&self, subject: &str) -> Option<i32> {
        let mut locator = self.subjects.locator();
        locator.run(subject).map(|idx| idx as i32)
    }

    pub fn get_predicate_idx(&self, predicate: &str) -> Option<i32> {
        let mut locator = self.predicates.locator();
        locator.run(predicate).map(|idx| idx as i32)
    }

    pub fn get_object_idx(&self, object: &str) -> Option<i32> {
        let mut locator = self.objects.locator();
        locator.run(object).map(|idx| idx as i32)
    }
}
