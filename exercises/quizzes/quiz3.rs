use std::fmt::Display;

// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.
// TODO: Adjust the struct as described above.
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

enum Grade {
    Numeric(f64),
    Alphabetic(&'static str),
}
// impl Display for Grade {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Grade::Numeric(num) => write!(f, "{}", num), // Format numeric grades
//             // Grade::Alphabetic(v) => match (v.chars().next().unwrap(), v.chars().next().unwrap()) {
//             //     ()
//             // },
//             Grade::Alphabetic(v) => write!(f, "{}", v),
//         }

//         // match self {

//         // }
//     }
// }
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            // grade: Grade::Numeric(2.1),
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            // grade: Grade::Alphabetic("A+"),
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
