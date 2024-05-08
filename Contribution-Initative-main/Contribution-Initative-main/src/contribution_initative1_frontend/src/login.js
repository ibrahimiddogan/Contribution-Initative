import { contribution_initative1_backend } from "../../declarations/contribution_initative1_backend";

const btn_login = document.getElementById("btn_login");
const loginInfo = document.getElementById("login_info");

btn_login.addEventListener("click", async (e) => {
    e.preventDefault();
  
    const email = document.getElementById("email").value.toString();
    const password = document.getElementById("password").value.toString();

    try {
        const loginResult = await contribution_initative1_backend.login_user(email, password);
        if (loginResult.Ok) {
            loginInfo.textContent = "Successful login.";
            console.log("Successful login:", loginResult.Ok.Success);
            localStorage.setItem("loggedUser", email);
            // Anasayfaya yönlendirme
            window.location.href = "http://localhost:8082";

            // Kullanıcının emailini anasayfaya yazma
            
        } else {
            loginInfo.textContent = "Invalid email or password.";
            console.error("Invalid email or password:", loginResult.Error);
        }
    } catch (error) {
        loginInfo.textContent = error;
        console.error("Error during login:", error);
    }

});
