<script lang="ts">
  import FirmaView from "$lib/components/FirmaView.svelte";
  import AnsprechspersonView from "$lib/components/AnsprechspersonView.svelte";
  import KontaktView from "$lib/components/KontaktView.svelte";
  import AboutView from "$lib/components/AboutView.svelte";

  let activeTab = $state("firma");
  let isCollapsed = $state(false);

  function setTab(tab: string) {
    activeTab = tab;
  }
</script>

<div
  class="flex h-screen w-full bg-slate-50 text-slate-800 dark:bg-slate-900 dark:text-slate-100 font-sans overflow-hidden"
>
  <!-- Sidebar -->
  <aside
    class="{isCollapsed
      ? 'w-24'
      : 'w-64'} shrink-0 bg-white dark:bg-slate-800 shadow-xl border-r border-slate-200 dark:border-slate-700 flex flex-col z-10 transition-all duration-300 relative"
  >
    <div
      class="p-6 border-b border-slate-100 dark:border-slate-700 flex justify-center items-center h-28"
    >
      {#if isCollapsed}
        <img
          src="/LOGO.png"
          alt="Logo"
          class="w-full h-full max-h-16 object-contain mix-blend-multiply dark:mix-blend-normal"
        />
      {:else}
        <img
          src="/Banner.png"
          alt="MemoBase Contact Manager"
          class="w-full h-auto"
        />
      {/if}
    </div>

    <nav class="flex-1 p-4 flex flex-col space-y-2 {isCollapsed ? 'px-2' : ''}">
      <button
        class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 font-medium flex items-center {isCollapsed
          ? 'justify-center'
          : ''} {activeTab === 'firma'
          ? 'bg-teal-50 text-teal-700 dark:bg-teal-900/30 dark:text-teal-300 shadow-sm'
          : 'text-slate-600 hover:bg-slate-50 dark:text-slate-300 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white'}"
        onclick={() => setTab("firma")}
        title="Firmen"
      >
        <img
          src="/COMPANY.png"
          alt="Firmen"
          class="{!isCollapsed ? 'mr-3' : ''} w-10 h-10 object-contain"
        />
        {#if !isCollapsed}
          Firmen
        {/if}
      </button>

      <button
        class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 font-medium flex items-center {isCollapsed
          ? 'justify-center'
          : ''} {activeTab === 'ansprechsperson'
          ? 'bg-teal-50 text-teal-700 dark:bg-teal-900/30 dark:text-teal-300 shadow-sm'
          : 'text-slate-600 hover:bg-slate-50 dark:text-slate-300 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white'}"
        onclick={() => setTab("ansprechsperson")}
        title="Ansprechspersonen"
      >
        <img
          src="/PERSON.png"
          alt="Ansprechspersonen"
          class="{!isCollapsed ? 'mr-3' : ''} w-10 h-10 object-contain"
        />
        {#if !isCollapsed}
          Ansprechspersonen
        {/if}
      </button>

      <button
        class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 font-medium flex items-center {isCollapsed
          ? 'justify-center'
          : ''} {activeTab === 'kontakt'
          ? 'bg-teal-50 text-teal-700 dark:bg-teal-900/30 dark:text-teal-300 shadow-sm'
          : 'text-slate-600 hover:bg-slate-50 dark:text-slate-300 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white'}"
        onclick={() => setTab("kontakt")}
        title="Kontakte"
      >
        <img
          src="/CONTACT.png"
          alt="Kontakte"
          class="{!isCollapsed ? 'mr-3' : ''} w-10 h-10 object-contain"
        />
        {#if !isCollapsed}
          Kontakte
        {/if}
      </button>

      <div class="mt-auto pt-4">
        <button
          class="w-full text-center px-4 py-2 rounded-xl transition-all duration-200 text-sm font-medium {activeTab === 'about'
            ? 'bg-slate-200 dark:bg-slate-700 text-slate-800 dark:text-white shadow-sm'
            : 'text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-white'}"
          onclick={() => setTab("about")}
          title="About MemoBase"
        >
          {#if isCollapsed}
            ℹ️
          {:else}
            About MemoBase
          {/if}
        </button>
      </div>
    </nav>

    <div
      class="p-4 border-t border-slate-200 dark:border-slate-700 text-xs text-slate-400 flex items-center {isCollapsed
        ? 'justify-center'
        : 'justify-between'}"
    >
      {#if !isCollapsed}
        <span>v0.9.0</span>
      {/if}
      <button
        onclick={() => (isCollapsed = !isCollapsed)}
        class="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors flex items-center justify-center {isCollapsed
          ? 'w-full'
          : ''}"
      >
        {#if isCollapsed}
          <img src="/RIGHT.png" alt="Right" class="w-8 h-8" />
        {:else}
          <img src="/LEFT.png" alt="Left" class="w-8 h-8" />
        {/if}
      </button>
    </div>
  </aside>

  <!-- Main Content -->
  <main
    class="flex-1 overflow-y-auto bg-slate-50/50 dark:bg-[#0f172a] transition-all duration-300 p-8"
  >
    <div class="max-w-6xl mx-auto">
      {#if activeTab === "firma"}
        <FirmaView />
      {:else if activeTab === "ansprechsperson"}
        <AnsprechspersonView />
      {:else if activeTab === "kontakt"}
        <KontaktView />
      {:else if activeTab === "about"}
        <AboutView />
      {/if}
    </div>
  </main>
</div>
