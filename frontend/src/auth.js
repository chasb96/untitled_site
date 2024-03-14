export default class AuthService {
    setToken(token) {
        localStorage.setItem("token", token);

        let user = JSON.parse(atob(token.split('.')[1]));

        localStorage.setItem("user_id", '' + user.user_id);
        localStorage.setItem("username", user.username);

        window.dispatchEvent(new Event("token-change"));
    }

    getToken() {
        return localStorage.getItem("token");
    }

    getUserId() {
        return parseInt(localStorage.getItem("user_id"));
    }

    getUsername() {
        return localStorage.getItem("username");
    }

    isAuthenticated() {
        let token = this.getToken();

        return token != null && token != "";
    }
}