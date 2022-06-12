#[derive(Clone,Debug)]
struct User {name: String}

trait UserRepository {
    fn get_all_user(&self) -> Vec<User>;
}

struct UserInteractor<T>
where
    T : UserRepository
{
    user_repository : T,
}

impl<T> UserInteractor<T>
where
    T : UserRepository
{
    fn get_all_user(&self) -> Vec<User> {
        self.user_repository.get_all_user()
    }
}

struct DBResult {
    users : Vec<User>,
}


impl DBResult {
    pub fn new() -> Self {
        Self {
            users : vec![
                User{name:String::from("take")},
                User{name:String::from("mike")},
                User{name:String::from("kouji")},
            ]
        }
    }
}

impl UserRepository for DBResult {
    fn get_all_user(&self) -> Vec<User> {
        let mut v = Vec::new();
        for user in self.users.iter() {
            let user = user.clone();
            v.push(user);
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::{User, UserRepository, DBResult, UserInteractor};

    struct TestResult{users:Vec<User>}

    impl  TestResult {
        pub fn new() -> Self {
            Self {
                users : vec![
                    User{name:String::from("test4")},
                    User{name:String::from("test5")},
                    User{name:String::from("test6")},
                ]
            }
        } 
    }

    impl UserRepository for TestResult {
        fn get_all_user(&self) -> Vec<User> {
            let mut v = Vec::new();
            for user in self.users.iter() {
                let user = user.clone();
                v.push(user);
            }
            v
        }
    }

    #[test]
    fn test1() {
        let ui = UserInteractor{
            user_repository : DBResult::new(),
        }; 
        let ui = ui.get_all_user();
        let mut uiter = ui.iter();
        assert_eq!(uiter.next().unwrap().name,String::from("take"));
        assert_eq!(uiter.next().unwrap().name,String::from("mike"));
        assert_eq!(uiter.next().unwrap().name,String::from("kouji"));
    }

    #[test]
    fn test2() {
        let ui = UserInteractor{
            user_repository : TestResult::new()
        }; 
        let ui = ui.get_all_user();
        let mut uiter = ui.iter();
        assert_eq!(uiter.next().unwrap().name,String::from("test4"));
        assert_eq!(uiter.next().unwrap().name,String::from("test5"));
        assert_eq!(uiter.next().unwrap().name,String::from("test6"));
    }
}