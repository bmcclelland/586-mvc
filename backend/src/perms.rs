//use common::*;

#[derive(Debug,PartialEq)]
pub enum Perm {
    ReadWorker,
    ReadProject,
    ReadTask,
    DeleteWorker,
    DeleteProject,
    DeleteTask,
    CreateWorker,
    CreateProject,
    CreateTask,
    ModifyWorker,
    ModifyProject,
    ModifyTask,
}

// Permission requirements. Actions have these.
#[derive(Debug,PartialEq)]
pub enum PermReq {
    And(Vec<PermReq>),
    Or(Vec<PermReq>),
    Just(Perm),
}

impl PermReq {
    // Permission requirements may be satisfied by some set of permissions.
    pub fn satisfied_by(&self, perms: &Vec<Perm>) -> bool {
        use PermReq::*;
        match self {
            And(reqs) => {
                reqs.iter().all(|r| r.satisfied_by(perms))
            }
            Or(reqs) => {
                reqs.iter().any(|r| r.satisfied_by(perms))
            }
            Just(p) => { 
                perms.contains(p) 
            }
        }
    }
}

macro_rules! perms(
    ($p: ident) => {
        PermReq::Just(Perm::$p)
    };
    
    (($($ps: tt)+)) => {
        perms!($($ps)+)
    };

    ($p: tt $(& $ps:tt)*) => {
        PermReq::And(vec![perms2!($p) $(,perms2!($ps))*])
    };
    
    ($p: tt $(| $ps:tt)*) => {
        PermReq::Or(vec![perms2!($p) $(,perms2!($ps))*])
    };
);

#[allow(unused)]
macro_rules! perms2(
    ($p: ident) => {
        PermReq::Just(Perm::$p)
    };
    
    (($($ps: tt)+)) => {
        perms!($($ps)+)
    };
);


#[cfg(test)]
mod tests {
    use super::*;
    use PermReq::*;
    use Perm::*;
        
    macro_rules! sat(
        ($reqs: tt by $perms: tt) => {
            assert!(perms! $reqs . satisfied_by(&vec! $perms));
        }
    );
    
    macro_rules! unsat(
        ($reqs: tt by $perms: tt) => {
            assert!(!perms! $reqs . satisfied_by(&vec! $perms));
        }
    );

    #[test]
    fn perm_logic() {
        assert_eq!(
            perms!(CreateWorker),
            Just(CreateWorker)
        );
        
        assert_eq!(
            perms!((CreateWorker)),
            Just(CreateWorker)
        );
        
        assert_eq!(
            perms!(CreateWorker & DeleteWorker),
            And(vec![Just(CreateWorker), Just(DeleteWorker)])
        );
        
        assert_eq!(
            perms!((CreateWorker & DeleteWorker)),
            And(vec![Just(CreateWorker), Just(DeleteWorker)])
        );
        
        assert_eq!(
            perms!(CreateWorker | DeleteWorker),
            Or(vec![Just(CreateWorker), Just(DeleteWorker)])
        );
        
        assert_eq!(
            perms!((CreateProject | DeleteProject) 
                    & (CreateWorker & DeleteWorker)),
            And(vec![
                Or(vec![Just(CreateProject), Just(DeleteProject)]),
                And(vec![Just(CreateWorker), Just(DeleteWorker)])
            ])
        );
    }

    #[test]
    fn perm_satisfy() {
        sat!((CreateWorker)
            by [DeleteWorker, ReadWorker, CreateWorker]
        );

        sat!((CreateWorker & DeleteWorker)
            by [DeleteWorker, ReadWorker, CreateWorker]
        );
        
        sat!((CreateWorker | DeleteWorker)
            by [DeleteWorker, ReadWorker]
        );
        
        sat!((CreateWorker | DeleteWorker)
            by [ReadWorker, CreateWorker]
        );
       

        unsat!((CreateWorker)
            by [DeleteWorker, ReadWorker]
        );

        unsat!((CreateWorker & DeleteWorker)
            by [ReadWorker, CreateWorker]
        );
        
        unsat!((CreateWorker | DeleteWorker)
            by [ReadWorker]
        );
    }
}
