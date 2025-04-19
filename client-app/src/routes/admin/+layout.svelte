<script>
  let menus = [
    { name: "Dashboard", link: "/admin/dashboard" },
    { name: "Users", link: "/admin/users" },
    { name: "Settings", link: "/admin/settings" },
  ];

  function logout() {
    // Handle logout logic here
    console.log("Logged out");
  }
</script>

<div class="layout">
  <div class="header">
    <div class="logo">Admin Panel</div>
    <button class="logout" on:click={logout}>Logout</button>
  </div>
  <div class="sidebar">
    {#each menus as menu}
      <div class="menu-item">
        <a href={menu.link}>{menu.name}</a>
      </div>
    {/each}
  </div>
  <div class="content">
    <slot />
  </div>
</div>

<style>
  .layout {
    display: grid;
    grid-template-rows: auto 1fr;
    grid-template-columns: 250px 1fr;
    height: 100vh;
  }

  .header {
    grid-column: 1 / -1;
    background-color: #333;
    color: white;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .logout {
    cursor: pointer;
    background: none;
    border: none;
    color: white;
    font-size: 1rem;
  }

  .sidebar {
    background-color: #f4f4f4;
    padding: 1rem;
    border-right: 1px solid #ddd;
  }

  .menu-item {
    margin-bottom: 1rem;
  }

  .menu-item a {
    text-decoration: none;
    color: #333;
    font-weight: bold;
  }

  .content {
    padding: 1rem;
    overflow-y: auto;
  }
</style>
