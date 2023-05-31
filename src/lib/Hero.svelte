<script>
  import Icon from '$lib/Icon.svelte';
</script>


<section>

  <div class="middle">
  <div class="left">
    <h1>AdGuardian</h1>
    <p class="subtitle">
      Terminal-based, real-time traffic monitoring and statistics for your
      <a href="https://github.com/AdguardTeam/AdGuardHome">AdGuard Home</a> instance
    </p>
    <img width="256" src="/images/ferris.png" />
    <a class="btn-link" href="https://github.com/lissy93/adguardian-term">
      <Icon name="github" width="20" height="20" />
      View on GitHub
    </a>
    <div class="links">
      <a href="https://lissy93.github.io/AdGuardian-Term">
        <Icon name="rust" width="20" height="20" />
        <span>Rust Doc</span>
      </a>
      <a href="https://crates.io/crates/adguardian">
        <Icon name="crates" width="20" height="20" />
        <span>Crates.io</span>
      </a>
      <a href="https://hub.docker.com/r/lissy93/adguardian">
        <Icon name="docker" width="20" height="20" />
        <span>DockerHub</span>
      </a>
      <a href="https://codeberg.org/alicia/adguardian">
        <Icon name="codeberg" width="20" height="20" />
        <span>CodeBerg Mirror</span>
      </a>
    </div>
  </div>
  <div class="right">
    <img class="screenshot" src="/images/screen-record.gif" />
  </div>
  </div>
  <p class="license">
    Free & Open Source |
    Licensed under 
    <a href="https://github.com/Lissy93/AdGuardian-Term/blob/main/LICENSE">MIT</a>
    &copy; 2023 <a href="https://aliciasykes.com">Alicia Sykes</a>
  </p>
</section>


<style lang="scss">
section {
  background: var(--bg-1);
  background: linear-gradient(315deg, var(--bg-1) 65%, var(--bg-2) 100%);
  color: var(--fg);
  padding: 1rem;
  min-height: 100%;
  display: flex;
  flex-direction: column;
  .middle {
    margin-top: 8%;
    display: flex;
    justify-content: space-around;
    flex-wrap: wrap;
  }
  
  h1 {
    margin: 0;
    color: var(--primary);
    font-size: 4rem;
    font-weight: 600;
    font-family: 'RobotoSlab';
    text-shadow: 2px 2px 3px #000;
  }
  .subtitle {
    font-size: 1.2rem;
    font-weight: 400;
    font-family: 'JetBrains Mono';
    text-align: center;
    margin: 0.5rem auto;
    color: var(--primary);
    max-width: 600px;
    text-shadow: 2px 2px 3px #000;
    a { color: var(--primary); }
  }
  a.btn-link {
      color: var(--primary);
      text-decoration: none;
      margin: 0.5rem;
      padding: 0.5rem;
      border-radius: 0.25rem;
      border: 1px solid var(--primary);
      transition: all 0.2s ease-in-out;
      min-width: 6rem;
      text-align: center;
      font-family: 'RobotoSlab';
      font-weight: bold;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      &:hover {
        background: var(--primary);
        color: var(--bg-1);
      }
    }
  .links {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-evenly;
    max-width: 600px;
    gap: 1.25rem;
    margin: 1rem auto;
    a {
      color: var(--primary);
      opacity: 0.5;
      font-family: 'JetBrains Mono';
      transition: all 0.2s ease-in-out;
      text-decoration: none;
      display: flex;
      align-items: center;
      gap: 0.25rem;
      &:hover {
        opacity: 1;
        span {
          text-decoration: underline;
        }
      }
    }
    
  }
  .license {
    color: var(--grey);
    font-family: 'JetBrains Mono';
    margin: 1rem auto;
    font-size: 0.75rem;
    opacity: 0.5;
    text-align: center;

    @media (min-width: 800px) {
      bottom: 1rem;
      position: absolute;
      width: -moz-available;
      margin: 0 auto;
      display: flex;
      justify-content: center;
    }
    a {
      color: var(--grey);
      text-decoration: none;
      margin: 0 6px;
    }
  }
  .left {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .left, .right {
    max-width: 800px;
    @media (min-width: 800px) {
      width: 50%;
    }
  }
  .screenshot {
    width: 100%;
  }
}
</style>
