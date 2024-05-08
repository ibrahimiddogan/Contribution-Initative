import { contribution_initative1_backend } from "../../declarations/contribution_initative1_backend";
document.addEventListener("DOMContentLoaded", async function() {
    const loginUser = document.getElementById("loginUser");
    const userEmail = localStorage.getItem("loggedUser");


const userData=await contribution_initative1_backend.get_user_by_email(userEmail)
console.log(userData)
document.getElementById('total_donate').textContent = userData[0].totalDonate + '$'
document.getElementById('name').textContent = userData[0].name
document.getElementById('last_name').textContent = userData[0].lastname
document.getElementById('birthday').textContent = userData[0].birthday
document.getElementById('email').textContent = userData[0].email

    
  
    if (userEmail) {
      // Kullanıcı giriş yapmış, emaili göster
      loginUser.textContent = userEmail;
  
      const signOutButton = document.createElement("a");
      signOutButton.textContent = "Sign Out";
      signOutButton.classList.add("ml-4");
      signOutButton.classList.add("text-red-500");
      signOutButton.classList.add("cursor-pointer");
      signOutButton.addEventListener("click", function() {
        event.preventDefault(); // Prevent the default behavior
        localStorage.removeItem("loggedUser");
        location.href='http://localhost:8082' 
      });
      loginUser.insertAdjacentElement("beforeend", signOutButton);
    } else {
      // Kullanıcı giriş yapmamış, login linki göster
      loginUser.textContent = "Login";
      loginUser.href = "profile.html";
    }
  
    // Register metnini gizle
    document.getElementById("registerUser").classList.add("hidden");
  });
  