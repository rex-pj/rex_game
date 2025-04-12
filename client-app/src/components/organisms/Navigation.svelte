<script>
  let navItems = [
    { name: "Flashcard", href: "/flashcard", actived: true },
    { name: "Vật phẩm", href: "/items" },
  ];
</script>

<nav class="navbar navbar-expand-lg navbar-light topbar">
  <div class="container">
    <a class="navbar-brand" href="/"
      ><enhanced:img src="../../assets/imgs/logo.png" alt="logo" /> Rex Game</a
    >
    <button
      class="navbar-toggler"
      type="button"
      data-bs-toggle="collapse"
      data-bs-target="#navbarNav"
      aria-controls="navbarNav"
      aria-expanded="false"
      aria-label="Toggle navigation"
    >
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
      <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
        {#each navItems as item}
          <li class="nav-item {item.actived ? 'active' : ''}">
            <a class="nav-link" href={item.href}>{item.name}</a>
          </li>
        {/each}
      </ul>
      <div class="d-flex">
        <button class="btn btn-primary me-2" type="button">Login</button>
        <button class="btn btn-secondary" type="button">Register</button>
      </div>
    </div>
  </div>
</nav>

<style>
  .topbar {
    background-color: var(--nav-bg-color);
    box-shadow: 0 0 10px rgb(26 26 26 / 15%);
  }

  .navbar-brand {
    color: #000;
    font-weight: 700;
    text-transform: uppercase;
  }

  .navbar-brand img {
    max-width: 40px;
    height: auto;
  }

  .nav-item.active .nav-link {
    color: var(--primary-color);
    font-weight: 700;
  }

  .nav-item .nav-link {
    color: #000;
  }

  .nav-item .nav-link {
    color: #000;
  }
</style>
