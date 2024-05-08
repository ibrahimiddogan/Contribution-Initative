import { contribution_initative1_backend } from "../../declarations/contribution_initative1_backend";

//donate-list
document.addEventListener("DOMContentLoaded", async function() {
  const listDonateGet = await contribution_initative1_backend.list_donate();
  const eUsersData= await contribution_initative1_backend.sort_esusers();
 console.log(eUsersData)
  const establishmentList = document.getElementById("establishment-list");
  eUsersData.map(function(item,i) {
    console.log(i)
      const div = document.createElement("div");
      div.className = "flex items-center gap-4 border-4 border-sky-600 rounded-xl px-4 py-4 w-[900px] bg-sky-600 shadow-md";
      div.innerHTML = `
          <img src="/eUserImage-${i}.webp" class="w-36" alt="">
          <div>
              <p class="text-sky-950 font-semibold">${item.name}</p>
              <p class="w-[600px] mt-1 text-stone-200 text-sm">${item.description}</p>
              <p class="w-[600px] mt-2.5 ">Total Donate: <i class="text-white">${item.totalDonate} $</i></p>
          </div>
          <button class="bg-green-500 text-slate-100 py-1.5 px-2.5 rounded-md">Donate</button>
      `;
      establishmentList.appendChild(div);
    });

  const donateList = document.getElementById("donate-list");

  const loginUser = document.getElementById("loginUser");
  const userEmail = localStorage.getItem("loggedUser");

  
  if (userEmail) {
    // Kullanıcı giriş yapmış, emaili göster
    loginUser.textContent = userEmail;
  
    loginUser.href = 'profile.html'; // Kullanıcı giriş yaptığında yönlendirilecek sayfa
    const signOutButton = document.createElement("a");
    signOutButton.textContent = "Sign Out";
    signOutButton.classList.add("ml-4");
    signOutButton.classList.add("text-red-500");
    signOutButton.classList.add("cursor-pointer");
    signOutButton.addEventListener("click", function() {
      event.preventDefault(); // Prevent the default behavior
      localStorage.removeItem("loggedUser");
      location.reload(); // Sayfayı yenile
    });
    loginUser.insertAdjacentElement("beforeend", signOutButton);
  
    // Register metnini gizle
    document.getElementById("registerUser").classList.add("hidden");
  } else {
    // Kullanıcı giriş yapmamış, login linki göster
    loginUser.textContent = "Login";
    loginUser.href = "login.html";
  }

  


  listDonateGet.forEach(function(item) {
    const li = document.createElement("li");
    li.className = "border-l-4 border-primary px-4 pl-4 text-sm py-2";
    li.textContent = `${item.donor_user} Donated ${item.amount}₺ to the ${item.donor_establishment} organization.`;
    donateList.appendChild(li);
  });
  //donate-list end
  

  const listDonateGetTop = await contribution_initative1_backend.list_top_donors();
  console.log('top_donate',listDonateGetTop)
  const topDonate = document.getElementById("top-donate");
  topDonate.innerHTML =  `<p class="text-yellow-300   text-sm border-2 border-darkBlue border-b-2 border-b-yellow-200 py-2 px-6 shadow-xl "><b class="text-lg">🏆</b> The Person Who Donated The Most Today: <i class=" tracking-widest ml-2  text-yellow-200"> ${listDonateGetTop[0]} </i> </p>`

 
});
