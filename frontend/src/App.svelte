<script lang="ts">
  import Header from './components/Header.svelte';
  import Footer from './components/Footer.svelte';
  import Landing from './pages/Landing.svelte';
  import SearchPage from './pages/Search.svelte';
  import Profile from './pages/Profile.svelte';

  import Router from 'svelte-spa-router';

  const routes = {
    '/': Landing,
    '/search/:player': SearchPage,
    '/profile/:slug': Profile
  };
</script>

<Header />
<Router {routes} />
<Footer />
