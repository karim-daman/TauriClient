<script>
  import "carbon-components-svelte/css/all.css";
  import Router from "svelte-spa-router";
  import routes from "./routes";
  import { t } from "./locale/i18n";

  import {
    Theme,
    RadioButtonGroup,
    RadioButton,
    Header,
    HeaderNav,
    HeaderNavItem,
    HeaderNavMenu,
    SideNav,
    SideNavItems,
    SideNavMenu,
    SideNavMenuItem,
    SideNavLink,
    SideNavDivider,
    SkipToContent,
    Content,
    Grid,
    Row,
    Column,
  } from "carbon-components-svelte";
  // import Fade from "carbon-icons-svelte/lib/Fade.svelte";
  import IcnTest from "$icons/icn_test.svelte";

  let isSideNavOpen = false;

  let theme = "white"; // "white" | "g10" | "g80" | "g90" | "g100"

  $: document.documentElement.setAttribute("theme", theme);

  import { onMount } from "svelte";
  import toast, { Toaster } from "svelte-french-toast";

  onMount(() => {
    toast.success("It works!");
  });
</script>

<Toaster />

<Theme bind:theme persist persistKey="__carbon-theme" />

<main>
  <Header company="IBM" platformName="Carbon Svelte" bind:isSideNavOpen>
    <svelte:fragment slot="skip-to-content">
      <SkipToContent />
    </svelte:fragment>
    <HeaderNav>
      <HeaderNavItem href="/" text="Link 1" />
      <HeaderNavItem href="/" text="Link 2" />
      <HeaderNavItem href="/" text="Link 3" />
      <HeaderNavMenu text="Menu">
        <HeaderNavItem href="/" text="Link 1" />
        <HeaderNavItem href="/" text="Link 2" />
        <HeaderNavItem href="/" text="Link 3" />
      </HeaderNavMenu>
      <HeaderNavItem href="/" text="Link 4" />
    </HeaderNav>
  </Header>

  <SideNav bind:isOpen={isSideNavOpen} rail>
    <SideNavItems>
      <SideNavLink icon={IcnTest} text="Link 1" href="/" isSelected />
      <SideNavLink icon={IcnTest} text="Link 2" href="/" />
      <SideNavLink icon={IcnTest} text="Link 3" href="/" />
      <SideNavMenu icon={IcnTest} text="Menu">
        <SideNavMenuItem href="/" text="Link 1" />
        <SideNavMenuItem href="/" text="Link 2" />
        <SideNavMenuItem href="/" text="Link 3" />
      </SideNavMenu>
      <SideNavDivider />
      <SideNavLink icon={IcnTest} text="Link 4" href="/" />
    </SideNavItems>
  </SideNav>

  <Content>
    <Grid>
      <Row>
        <Column>
          <h1>{@html $t("greeting")}</h1>

          <RadioButtonGroup legendText="Carbon theme" bind:selected={theme}>
            {#each ["white", "g10", "g80", "g90", "g100"] as value}
              <RadioButton labelText={value} {value} />
            {/each}
          </RadioButtonGroup>

          <Router {routes} />
        </Column>
      </Row>
    </Grid>
  </Content>
</main>
