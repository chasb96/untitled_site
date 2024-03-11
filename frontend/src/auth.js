export default class AuthService {
    setToken(token) {
        localStorage.setItem("token", token);

        window.dispatchEvent(new Event("token-change"));
    }

    getToken() {
        return localStorage.getItem("token");
    }

    isAuthenticated() {
        let token = this.getToken();

        return token != null && token != "";
    }
}