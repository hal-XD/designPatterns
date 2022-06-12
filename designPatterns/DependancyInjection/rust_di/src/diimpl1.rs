
#[derive(Clone,Debug)]
struct User {name: String}

trait UserRepository {
    fn get_all_user(&self) -> Vec<User>;
}

pub struct DBResult {
    users : Vec<User>
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

impl DBResult {
    // 実際はDBから結果を取得したりするが簡略化する
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

pub struct UserInteractor {
    user_repository: Box<dyn UserRepository>,
}

impl UserInteractor {
    pub fn get_all_user(&self) -> Vec<User> {
        self.user_repository.get_all_user()
    }
}

#[cfg(test)]
mod test {
    use super::{User, UserInteractor, UserRepository, DBResult};

    struct TestResult {
        users : Vec<User>
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

    impl TestResult {
        // 実際はDBから結果を取得したりするが簡略化する
        pub fn new() -> Self {
            Self {
                users : vec![
                    User{name:String::from("test1")},
                    User{name:String::from("test2")},
                    User{name:String::from("test3")},
                ]
            }
        }
    }


    #[test]
    fn test1() {
        let ui = UserInteractor{
            user_repository : Box::new(DBResult::new()),
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
            user_repository : Box::new(TestResult::new()),
        }; 
        let ui = ui.get_all_user();
        let mut uiter = ui.iter();
        assert_eq!(uiter.next().unwrap().name,String::from("test1"));
        assert_eq!(uiter.next().unwrap().name,String::from("test2"));
        assert_eq!(uiter.next().unwrap().name,String::from("test3"));
    }
}