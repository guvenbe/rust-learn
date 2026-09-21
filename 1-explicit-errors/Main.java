package trycatch;

public class User {
    final String name;
    User(String name) {
        this.name = name;
    }
}

class UserNotFoundException extends Exception {
    UserNotFoundException(String message) {
        super(message);
    }
}

class UserService {
        User findUserById(int id) throws UserNotFoundException {
        if (id == 1) {
            return new User("Alice");
        }
        throw new UserNotFoundException("User not found");
    }
}

public class Main {
    public static void main(String[] args) {
        try {
            UserService service = new UserService();
            User user = service.findUserById(42);
            System.out.println("User: " + user.name);
        } catch (UserNotFoundException e) {
            System.err.println("Application error: " + e.getMessage());
        }
    }
} {
    
}
