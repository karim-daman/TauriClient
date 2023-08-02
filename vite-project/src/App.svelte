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
    HeaderUtilities,
    HeaderGlobalAction,
    HeaderAction,
    HeaderPanelLinks,
    HeaderPanelDivider,
    HeaderPanelLink,
    Modal,
    Button,
    ButtonSet,
  } from "carbon-components-svelte";
  // import Fade from "carbon-icons-svelte/lib/Fade.svelte";
  import IcnTest from "$icons/icn_test.svelte";
  import IcnSettings from "$icons/icn_settings.svelte";

  let ref = null;

  let isSideNavOpen = false;

  let theme = "white"; // "white" | "g10" | "g80" | "g90" | "g100"

  $: document.documentElement.setAttribute("theme", theme);

  import { onMount } from "svelte";
  import toast, { Toaster } from "svelte-french-toast";
  import LanguageSelect from "$lib/LanguageSelect.svelte";
  import ServerScan from "$lib/ServerScan.svelte";
  import DateTime from "$lib/DateTime.svelte";

  onMount(() => {
    toast.success("It works!");
  });

  let isOpen1 = false;
  let isOpen2 = false;
  let open = false;
</script>

<Toaster />

<Theme bind:theme persist persistKey="__carbon-theme" />

<main>
  <Header company="IBM" platformName="Carbon Svelte" bind:isSideNavOpen>
    <svelte:fragment slot="skip-to-content">
      <SkipToContent />
    </svelte:fragment>

    <HeaderNav>
      <!-- 
      <HeaderNavItem href="#/" text="Home" />
      <HeaderNavItem href="/" text="Link 2" />
      <HeaderNavItem href="/" text="Link 3" />
      <HeaderNavMenu text="Menu">
        <HeaderNavItem href="/" text="Link 1" />
        <HeaderNavItem href="/" text="Link 2" />
        <HeaderNavItem href="/" text="Link 3" />
      </HeaderNavMenu>
      <HeaderNavItem href="/" text="Link 4" /> 
    -->
    </HeaderNav>
  </Header>

  <SideNav bind:isOpen={isSideNavOpen} rail>
    <SideNavItems>
      <SideNavLink icon={IcnTest} text="Home" href="#/" isSelected />
      <SideNavLink icon={IcnTest} text="Link 2" href="/" />
      <SideNavLink icon={IcnTest} text="Link 3" href="/" />
      <SideNavMenu icon={IcnTest} text="Menu">
        <SideNavMenuItem href="/" text="Link 1" />
        <SideNavMenuItem href="/" text="Link 2" />
        <SideNavMenuItem href="/" text="Link 3" />
      </SideNavMenu>
      <SideNavDivider />
      <div class="absolute bottom-0 inset-x-0">
        <SideNavLink icon={IcnTest} text="Link 4" href="/" />
        <SideNavLink icon={IcnTest} text="Link 4" href="/" />
        <SideNavLink icon={IcnTest} text="Link 4" href="/" />
        <SideNavLink icon={IcnSettings} text="Settings" href="#/" on:click={() => (open = true)} />
      </div>
    </SideNavItems>
  </SideNav>

  <Modal bind:open modalHeading="Create database" primaryButtonText="Confirm" secondaryButtonText="Cancel" on:click:button--secondary={() => (open = false)} on:open on:close on:submit>
    <RadioButtonGroup legendText="Carbon theme" bind:selected={theme}>
      {#each ["white", "g10", "g80", "g90", "g100"] as value}
        <RadioButton labelText={value} {value} />
      {/each}
    </RadioButtonGroup>
  </Modal>

  <Content>
    <Grid>
      <Row>
        <Column>
          <!-- <h1>{@html $t("greeting")}</h1> -->

          <Router {routes} />
        </Column>
      </Row>
    </Grid>
  </Content>
</main>

<style>
  .border {
    border-style: solid;
    border-width: 0.5px;
  }
</style>
