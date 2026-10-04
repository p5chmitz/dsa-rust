/// This module represents a weak partial order, meaning not all
/// students are comparable, and some can be equivalent. This
/// implementation illustrates a workaround to guarantee global
/// reflexivity over the floating point "gotcha" by performing a
/// memory address comparison. In this way, Students are reflexive,
/// even if they have NaN grades, but not comparable if two students
/// have NaN grades.
pub mod partial_order {
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub struct Student {
        pub name: &'static str,
        pub grade: f32,
    }

    // The PartialEq trait is manually implemented instead of derived
    // to override the default component-wise struct field comparison
    // logic. The eq method specifies a partial equivalence relation
    // in which two objects are only equal if their grades
    // are strictly equal.
    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            // Guarantees global reflexivity by only establishing
            // equivalence for NANs if they represent the exact
            // same memory address
            if std::ptr::eq(self, other) {
                return true;
            }

            self.grade.eq(&other.grade)
        }
    }

    // The PartialOrd impl must be semantically consistent
    // with the PartialEq impl so only grades are considered,
    // and because not all floats are directly comparable, the
    // comparison may return None.
    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if std::ptr::eq(self, other) {
                return Some(Ordering::Equal);
            }

            self.grade.partial_cmp(&other.grade)
        }
    }
}

/// This module represents a total preorder implementation
pub mod total_preorder {
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub struct Student {
        pub name: &'static str,
        pub grade: f32,
    }

    /// An equivalence class to categorize grades based
    /// on ranges of percentage point values.
    /// Derives PartialEq to reduce boilerplate.
    /// In Rust, the ordering of the discrimiants is important,
    /// such that the first listed discrimiant carries a value
    /// of 0, the next discrimiant a 1, and so on.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Grade {
        INC,    // 0
        F,      // 1
        DMinus, // 2
        D,
        DPlus,
        CMinus,
        C,
        CPlus,
        BMinus,
        B,
        BPlus,
        AMinus,
        A,
        APlus,
    }
    // Gets rid of the possibility of None for a total preorder,
    // adds cases where NAN and _ == INC
    impl Grade {
        /// Maps a score into its corresponding equivalence class
        pub fn eq_class(value: f32) -> Grade {
            match value {
                val if val.is_nan() => Grade::INC,
                //..59.5 => Grade::F,
                //59.5..63.0 => Grade::DMinus,
                //63.0..66.5 => Grade::D,
                //66.5..69.5 => Grade::DPlus,
                //69.5..73.0 => Grade::CMinus,
                //73.0..76.5 => Grade::C,
                //76.5..79.5 => Grade::CPlus,
                //79.5..83.0 => Grade::BMinus,
                //83.0..86.5 => Grade::B,
                //86.5..89.5 => Grade::BPlus,
                //89.5..93.0 => Grade::AMinus,
                //93.0..96.5 => Grade::A,
                //96.5.. => Grade::APlus,
                ..59.5 => Grade::F,

                59.5..62.8 => Grade::DMinus,
                62.8..66.1 => Grade::D,
                66.1..69.5 => Grade::DPlus,

                69.5..72.8 => Grade::CMinus,
                72.8..76.1 => Grade::C,
                76.1..79.5 => Grade::CPlus,

                79.5..82.8 => Grade::BMinus,
                82.8..86.1 => Grade::B,
                86.1..89.5 => Grade::BPlus,

                89.5..92.8 => Grade::AMinus,
                92.8..96.1 => Grade::A,
                96.1.. => Grade::APlus,

                _ => Grade::INC, // catch-all because the compiler doesn't trust floats
            }
        }
    }

    // Implements float equality by overloading the == operator.
    // PartialEq is derivable, but enforces that all struct fields
    // are Eq, which may not be desired.
    //
    // Compares rounded scores to induce a preorder.
    // NOTE: Must match the logic in Ord::cmp for consistency!!
    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            //self.grade.round() as i32 == other.grade.round() as i32
            Grade::eq_class(self.grade) == Grade::eq_class(other.grade)
        }
    }

    // Implements Eq because reflexivity cannot be derived for floats
    impl Eq for Student {}

    // Imposes a generic comparison because the
    // comparison is actually defined in cmp() for Ord.
    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(Ord::cmp(self, other)) // Verbose expression
                                        //Some(self.cmp(other)) // If you wanna be slick
        }
    }

    // Actual comparison implementation for a preorder.
    impl Ord for Student {
        // Implements a total preorder
        // Only the grade's equivalence class is compared
        // such that 77.5 effectively equials 79.1
        fn cmp(&self, other: &Self) -> Ordering {
            // The actual f32 comparison logic, rounded to the nearest ingeter
            //std::intrinsics::three_way_compare(self, other) // Unavailable to us plebs
            //(self.grade.round() as i32).cmp(&(other.grade.round() as i32))
            Grade::eq_class(self.grade).cmp(&Grade::eq_class(other.grade))
        }
    }
}

/// This module represents a strict total order implementation
pub mod total_order {
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub struct Student {
        pub name: &'static str,
        pub grade: f32,
    }

    // Defines a full equivalence realtion for Student where two Students
    // are only equal if their rounded grade and lexicographic names are equal.
    // PartialEq can be derived, but would use structural equality,
    // which may not match the intended equivalence relation.
    // NOTE: Must remain consistent with Ord::cmp if both are implemented.
    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            //self.cmp(other) == Ordering::Equal // Same as deriving the trait
            self.grade.round().total_cmp(&other.grade.round()) == Ordering::Equal
                && self.name.cmp(other.name) == Ordering::Equal
        }
    }

    // Eq marks that equality is reflexive under the PartialEq impl.
    // Cannot be derived because Eq is not implemented for floats.
    impl Eq for Student {}

    // Delegates ordering to Ord::cmp.
    // NOTE: Though the code compiles, Rust does not like it when you derive
    // PartialOrd when implementing Ord
    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // First compares grades rounded to the nearest whole number,
    // and then breaks ties by comparaing the names lexicographically.
    impl Ord for Student {
        fn cmp(&self, other: &Self) -> Ordering {
            let grade_order = self.grade.round().total_cmp(&other.grade.round());
            if grade_order == std::cmp::Ordering::Equal {
                self.name.cmp(other.name)
            } else {
                grade_order
            }
        }
    }
}

#[test]
/// A partial order induces transitivity, reflexivity,
/// and antisymmetry. In this non-strict partial order, only rounded
/// grades get compared.
fn partial_order() {
    use partial_order::Student;
    use std::cmp::Ordering;

    // Partial order
    let a = Student {
        name: "Dichael",
        grade: 78.8,
    };
    let b = Student {
        name: "Ephraim",
        grade: 78.8,
    };
    let c = Student {
        name: "Froedrik",
        grade: 78.8,
    };
    let d = Student {
        name: "Dingus",
        grade: 79.7,
    };
    let e = Student {
        name: "Ephraim",
        grade: 81.1,
    };
    let m = Student {
        name: "Dingus",
        grade: f32::NAN, // Incomplete/INC
    };
    let n = Student {
        name: "Dangus",
        grade: f32::NAN, // Incomplete/INC
    };

    // Transitivity
    // if a <= b and b <= c, then a <= c
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&c), Some(Ordering::Equal));
    assert_eq!(a.partial_cmp(&c), Some(Ordering::Equal));
    // Mixed transitivity
    // if a == b and b < c, then a < c
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&d), Some(Ordering::Less));
    // Directional chaining
    assert_eq!(e.partial_cmp(&d), Some(Ordering::Greater));
    assert_eq!(d.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(e.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(c.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(d.partial_cmp(&e), Some(Ordering::Less));
    assert_eq!(c.partial_cmp(&e), Some(Ordering::Less));

    // Reflexivity
    // a == a
    assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&b), Some(Ordering::Equal));

    assert!(a.eq(&a)); // a == a
    assert!(a.eq(&b));
    assert!(b.eq(&a));
    assert!(n.eq(&n)); // NaN == NaN
    assert!(m.eq(&m)); // NaN == NaN
    assert!(!n.eq(&m)); // But it has to be the SAME NaN

    // Equivalence
    assert!(!a.eq(&m)); // 78.8 != NaN
                        // Indicates a lack of global reflexivity!
    assert!(!n.eq(&m)); // NaN != NaN

    // Antisymmetry
    // if a <= b and b <= a, then a == b
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&a), Some(Ordering::Equal));
    // Directionality check
    assert_eq!(a.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(d.partial_cmp(&a), Some(Ordering::Greater));

    // Incomparability (lack of totality)
    assert!(!a.eq(&m)); // Incomparability; 78.8 != NAN
    assert!(!m.eq(&n)); // Incomparability; NAN != NAN
    assert_eq!(a.partial_cmp(&m), None); // Incomparable!
    assert_eq!(m.partial_cmp(&a), None); // Symmetry of incomparability
    assert_eq!(m.partial_cmp(&n), None); // NAN != NAN
}

#[test]
/// This tests a total preorder which induces transitivity,
/// reflexivity, and comparability. In a total preorder, two elements
/// can be roughly equivalent without strict equality. In this non-strict
/// total preorder, two students can be equivalent if their scores fall
/// into the same letter grade.
fn tota_preorder() {
    use std::cmp::Ordering;
    use total_preorder::{Grade, Student};

    let a = Student {
        name: "Alonzo",
        grade: 78.8, // C+
    };
    let b = Student {
        name: "Brain",
        grade: 79.4, // C+
    };
    let c = Student {
        name: "Chamene",
        grade: 79.1, // C+
    };
    let d = Student {
        name: "Dichael",
        grade: 81.8, // B-
    };
    let e = Student {
        name: "Ephraim",
        grade: 83.7, // B
    };
    let m = Student {
        name: "Dingus",
        grade: f32::NAN, // INC
    };
    let n = Student {
        name: "Dangus",
        grade: f32::NAN, // INC
    };

    // Preliminary equivalence class testing
    assert_eq!(Grade::eq_class(a.grade), Grade::CPlus);
    assert_eq!(Grade::eq_class(b.grade), Grade::CPlus);
    assert_eq!(Grade::eq_class(c.grade), Grade::CPlus);
    assert_eq!(Grade::eq_class(d.grade), Grade::BMinus);
    assert_eq!(Grade::eq_class(e.grade), Grade::B);
    assert_eq!(Grade::eq_class(m.grade), Grade::INC);
    assert_eq!(Grade::eq_class(n.grade), Grade::INC);

    // Illustrates discriminant values proving such
    // assertions as Grade::CPlus < Grade::B
    assert_eq!(Grade::INC as u8, 0);
    assert_eq!(Grade::CPlus as u8, 7);
    assert_eq!(Grade::A as u8, 12);
    assert!(Grade::CPlus < Grade::B);

    // Transitivity
    // if a <= b and b <= c, then a <= c
    //
    // Because of the rounding logic, 78.8, 79.4, and 79.1 are all
    // roughly equivalent, meaning that Alonzo, Brain, and Chamene
    // all reqpresent roughly equivalent Students.
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&c), Some(Ordering::Equal));
    assert_eq!(a.partial_cmp(&c), Some(Ordering::Equal));
    // Mixed transitivity
    // if a == b and b < c, then a < c
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&d), Some(Ordering::Less));
    // Directional chaining
    assert_eq!(e.partial_cmp(&d), Some(Ordering::Greater));
    assert_eq!(d.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(e.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(c.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(d.partial_cmp(&e), Some(Ordering::Less));
    assert_eq!(c.partial_cmp(&e), Some(Ordering::Less));

    // Reflexivity
    // a == a
    assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&b), Some(Ordering::Equal));

    // Totality/Comparability (with rough equivalence)
    // For all elements a & b, a <=b || b <= a (even NaNs!)
    assert!(a.eq(&b));
    assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    // Things that used to be incomparable, like NaN are now comparable
    // because PartialEq explicitly casts NAN as i32
    assert!(!a.eq(&m)); // 78.8 != NAN
    assert!(m.eq(&n)); // NAN as i32 == NAN as i32
    assert_eq!(a.partial_cmp(&m), Some(Ordering::Greater));
    assert_eq!(m.partial_cmp(&a), Some(Ordering::Less));
    assert_eq!(m.partial_cmp(&n), Some(Ordering::Equal)); // NaN is treated as equal to NaN

    // In a total preorder, equivalence does NOT imply identity
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    assert_ne!(a.name, b.name); // This proves it's a preorder, not a total order

    // Approximate comparability testing by fuzzing "all pairs"
    //use proptest::prelude::*;
    //proptest! {
    //    #[test]
    //    fn all_pairs_are_comparable(a in any::<Student>(), b in any::<Student>()) {
    //        prop_assert!(a.partial_cmp(&b).is_some());
    //        prop_assert!(b.partial_cmp(&a).is_some());
    //    }
    //}
}

#[test]
/// A total order induces reflexivity, transitivity,
/// antisymmetry, and comparability. In this non-strict total ordering,
/// each student is first compared based on rounded score, and totality
/// is induced via lexicographical name comparisons.
///
/// This test checks logic for both partial_cmp and cmp to illustrate that
/// both PartialOrd and Ord traits match predicate implementations.
fn total_order() {
    use std::cmp::Ordering;
    use total_order::Student;

    let a = Student {
        name: "Alonzo",
        grade: 78.8,
    };
    let b = Student {
        name: "Brain",
        grade: 79.4,
    };
    let c = Student {
        name: "Chamene",
        grade: 79.1,
    };
    let d = Student {
        name: "Dichael",
        grade: 81.8,
    };
    let e = Student {
        name: "Ephraim",
        grade: 83.7,
    };
    let m = Student {
        name: "Dingus",
        grade: f32::NAN,
    };
    let n = Student {
        name: "Dangus",
        grade: f32::NAN,
    };
    let q = Student {
        name: "Alonzo",
        grade: 78.8,
    };

    // Transitivity
    // if a <= b and b <= c, then a <= c
    //
    // Because of the rounding logic, 78.8, 79.4, and 79.1 are all
    // roughly equivalent, but the totality adds a tie breaker
    // by lexicographically comparing the name values.
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(b.partial_cmp(&c), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));

    assert_eq!(a.cmp(&b), Ordering::Less);
    assert_eq!(b.cmp(&c), Ordering::Less);
    assert_eq!(a.cmp(&c), Ordering::Less);

    // Mixed transitivity
    // if a == b and b < c, then a < c
    assert_eq!(a.partial_cmp(&q), Some(Ordering::Equal));
    assert_eq!(q.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&d), Some(Ordering::Less));

    assert_eq!(a.cmp(&q), Ordering::Equal);
    assert_eq!(q.cmp(&d), Ordering::Less);
    assert_eq!(a.cmp(&d), Ordering::Less);

    // Directional chaining
    assert_eq!(e.partial_cmp(&d), Some(Ordering::Greater));
    assert_eq!(d.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(e.partial_cmp(&c), Some(Ordering::Greater));
    assert_eq!(c.partial_cmp(&d), Some(Ordering::Less));
    assert_eq!(d.partial_cmp(&e), Some(Ordering::Less));
    assert_eq!(c.partial_cmp(&e), Some(Ordering::Less));

    assert_eq!(e.cmp(&d), Ordering::Greater);
    assert_eq!(d.cmp(&c), Ordering::Greater);
    assert_eq!(e.cmp(&c), Ordering::Greater);
    assert_eq!(c.cmp(&d), Ordering::Less);
    assert_eq!(d.cmp(&e), Ordering::Less);
    assert_eq!(c.cmp(&e), Ordering::Less);

    // Reflexivity
    // a == a
    assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    assert_eq!(b.partial_cmp(&b), Some(Ordering::Equal));

    assert_eq!(a.cmp(&a), Ordering::Equal);
    assert_eq!(b.cmp(&b), Ordering::Equal);

    // Antisymmetry
    // if a <= q AND q <= a, then a MUST EQUAL q
    // Antisymmetry is what separates this from a preorder
    //let a_le_q = a.partial_cmp(&q).map_or(false, |o| o.is_eq() || o.is_le());
    //let q_le_a = q.partial_cmp(&a).map_or(false, |o| o.is_eq() || o.is_le());
    let a_q = a.partial_cmp(&q).is_some_and(|o| o.is_eq() || o.is_le()); // a <= q
    let q_a = q.partial_cmp(&a).is_some_and(|o| o.is_eq() || o.is_le()); // q <= a

    if a_q && q_a {
        assert!(a.eq(&q));
        assert_eq!(a.name, q.name);
        assert_eq!(a.grade, q.grade);
    }

    // In a total order, equivalence implies identity
    assert!(a.eq(&q));
    assert_eq!(a.partial_cmp(&q), Some(Ordering::Equal));
    assert_eq!(a.cmp(&q), Ordering::Equal);
    assert_eq!(a.name, q.name);
    assert_eq!(a.grade, q.grade);

    // Totality/Comparability (with rough equivalence)
    // For all elements a & b, a <=b || b <= a implying that there cannot
    // be any None value for NaN comparisons
    assert!(a.eq(&q));
    assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    // Things that used to be incomparable, like NaN are now comparable
    // because PartialEq explicitly casts NAN as i32, however the total order
    // collapses the equivalence relation to an equality by lexicographically
    // ordering otherwise equivalent Students by name
    assert!(!m.eq(&n));
    assert!(a.partial_cmp(&n).is_some()); // Even disparate types are comparable
    assert_eq!(a.partial_cmp(&m), Some(Ordering::Less)); // Pass
                                                         //assert_eq!(m.partial_cmp(&a), Some(Ordering::Greater)); // FAIL???
    assert_eq!(m.partial_cmp(&n), Some(Ordering::Greater));

    //assert_eq!(a.cmp(&m), Ordering::Less);
    assert_eq!(m.cmp(&a), Ordering::Greater);
    assert_eq!(m.cmp(&n), Ordering::Greater);
}

// Requires total_cmp
pub fn okie(a: f32, b: f32) -> std::cmp::Ordering {
    // Illegal because NaN != NaN
    //a.cmp(&b)

    // Legal because total_cmp imposes a strict total order on floating point values
    a.total_cmp(&b)
}

/// Compares values
pub fn value_equality(a: &String, b: &String) -> bool {
    a.cmp(b) == std::cmp::Ordering::Equal
}
/// Compares pointers
pub fn ptr_equality(a: &String, b: &String) -> bool {
    std::ptr::eq(a, b)
}
#[test]
fn equal() {
    let a = &String::from("Peter");
    let b = &String::from("Peter");
    //assert_eq!(equality(a, b), std::cmp::Ordering::Equal);
    assert!(value_equality(a, b)); // Values are eqal

    let a = &String::from("Peter");
    let b = &String::from("Peter");
    assert!(!ptr_equality(a, b)); // Pointers are NOT equal
}

pub fn sortt<T: Ord>(list: &mut [T]) {
    list.sort()
}
#[test]
fn sortable() {
    let mut a = vec![2_i32, 3, 1];
    sortt(&mut a);
    assert_eq!(a, vec!(1, 2, 3));

    let mut b = [3_u8, 2, 1];
    sortt(&mut b);
    assert_eq!(b, [1, 2, 3]);

    let mut c = ["Peter", "Bobson", "Dangus"];
    sortt(&mut c);
    assert_eq!(c, ["Bobson", "Dangus", "Peter"]);
}
