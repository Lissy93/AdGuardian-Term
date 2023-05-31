
<svelte:head>
  <title>AdGuardian</title>
  <meta name="description" content="Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance" />
  <meta property="og:title" content="AdGuardian">
  <meta property="og:description" content="Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance">
  <meta property="og:image" content="https://i.ibb.co/QrHFjqb/adguardian-banner-1.png">
  <meta property="og:url" content="https://adguardian.as93.net">
  <meta name="twitter:card" content="summary_large_image">
</svelte:head>

<slot></slot>

<style lang="scss">
    @font-face {
      font-family: 'JetBrains Mono';
      src: url('/fonts/JetBrainsMono.ttf');
    }
    @font-face {
      font-family: 'RobotoSlab';
      src: url('/fonts/RobotoSlab.ttf');
    }

  :global(html) {
    --bg-1: #090f1b;
    --bg-2: #131a34;
    --bg-3: #101320;
    --fg: #fff;
    --primary: #17d569;
    --grey: #9f9f9f;
    font-family: 'Roboto', sans-serif;
    font-size: 16px;
    font-weight: 400;
    line-height: 1.5;
    text-align: left;
    min-height: 100%;
  }
  :global(html) {
    height: 100%;
  }
  :global(body) {
    margin: 0;
    height: 100%;
    background: var(--bg-1);
  }
  a {
      color: var(--primary);
    }
</style>
