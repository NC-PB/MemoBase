<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Ansprechsperson, Firma, Kontakt } from '$lib/types';
  import { onMount } from 'svelte';

  let personen = $state<Ansprechsperson[]>([]);
  let firmen = $state<Firma[]>([]);
  let allKontakte = $state<Kontakt[]>([]);
  let showModal = $state(false);
  let showDetailModal = $state(false);
  let isEditing = $state(false);
  let currentPerson = $state<Partial<Ansprechsperson>>({});
  let loading = $state(true);
  let searchQuery = $state('');

  let filteredPersonen = $derived(
    personen.filter(p => 
      p.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
      (p.vorname && p.vorname.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (p.mail && p.mail.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );

  let detailKontakte = $derived(allKontakte.filter(k => k.ansprechsperson_id === currentPerson.id).sort((a,b) => (b.datum || '').localeCompare(a.datum || '')));

  async function loadData() {
    loading = true;
    try {
      const [pRes, fRes, kRes] = await Promise.all([
        invoke<Ansprechsperson[]>('get_ansprechspersonen'),
        invoke<Firma[]>('get_firmen'),
        invoke<Kontakt[]>('get_kontakte')
      ]);
      personen = pRes;
      firmen = fRes;
      allKontakte = kRes;
    } catch (e) {
      console.error(e);
      alert('Failed to load data: ' + e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadData();
  });

  function openCreate() {
    isEditing = false;
    currentPerson = { anrede: '', name: '', vorname: '', tel_mobil: '', tel_direkt: '', mail: '', facebook: '', linkedin: '', xing: '', firma_id: undefined };
    showModal = true;
  }

  function openEdit(person: Ansprechsperson) {
    isEditing = true;
    currentPerson = { ...person };
    showModal = true;
    showDetailModal = false;
  }

  function openDetails(person: Ansprechsperson) {
    currentPerson = { ...person };
    showDetailModal = true;
  }

  async function deletePerson(id: number) {
    if (confirm('Person wirklich löschen? Diese Aktion ist endgültig.')) {
      try {
        await invoke('delete_ansprechsperson', { id });
        showDetailModal = false;
        await loadData();
      } catch (e) {
        console.error(e);
        alert('Delete failed.');
      }
    }
  }

  async function savePerson() {
    if (!currentPerson.name) {
      alert("Name ist erforderlich.");
      return;
    }
    // ensure firma_id is parsed to int
    if (currentPerson.firma_id) currentPerson.firma_id = parseInt(currentPerson.firma_id as any);
    
    try {
      if (isEditing) {
        await invoke('update_ansprechsperson', { person: currentPerson });
      } else {
        await invoke('create_ansprechsperson', { person: currentPerson });
      }
      showModal = false;
      await loadData();
    } catch (e) {
      console.error(e);
      alert('Save failed: ' + e);
    }
  }

  function getFirmaName(id?: number) {
    if (!id) return '-';
    return firmen.find(f => f.id === id)?.name || 'Unbekannt';
  }
</script>

<div class="flex flex-col space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
  <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
    <div>
      <h2 class="text-3xl font-bold tracking-tight text-slate-800 dark:text-slate-100">Ansprechspersonen</h2>
      <p class="text-slate-500 dark:text-slate-400 mt-2">Ihre direkten Kontakte verwalten.</p>
    </div>
    
    <div class="flex w-full md:w-auto items-center space-x-3">
      <div class="relative w-full md:w-64">
        <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-slate-400">🔍</span>
        <input 
          type="text" 
          bind:value={searchQuery}
          placeholder="Personen suchen..." 
          class="w-full pl-10 pr-4 py-2.5 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white transition-all shadow-sm"
        />
      </div>
      <button 
        onclick={openCreate}
        class="bg-teal-600 hover:bg-teal-700 text-white px-5 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all duration-200 font-medium flex items-center shrink-0"
      >
        <span class="mr-2 text-lg">+</span> Neue Person
      </button>
    </div>
  </div>

  <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-sm border border-slate-200 dark:border-slate-700 overflow-hidden">
    {#if loading}
      <div class="p-8 text-center text-slate-500">Lade Daten...</div>
    {:else if filteredPersonen.length === 0}
      <div class="p-12 text-center flex flex-col items-center">
        <div class="h-16 w-16 bg-slate-100 dark:bg-slate-700 rounded-full flex items-center justify-center mb-4 text-3xl">📭</div>
        <h3 class="text-lg font-medium text-slate-800 dark:text-slate-200">Keine Personen gefunden</h3>
        <p class="text-slate-500 mt-1">
          {#if searchQuery}Versuchen Sie einen anderen Suchbegriff.{:else}Fügen Sie Ihren ersten Ansprechpartner hinzu.{/if}
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 p-6">
        {#each filteredPersonen as person (person.id)}
          <div 
            class="group relative bg-slate-50 dark:bg-slate-900/50 rounded-2xl p-5 border border-slate-100 dark:border-slate-800 hover:shadow-md transition-all cursor-pointer"
            onclick={() => openDetails(person)}
          >
             <div class="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity flex space-x-1">
                <button onclick={(e) => { e.stopPropagation(); openEdit(person); }} class="p-1.5 text-slate-400 hover:text-teal-600 hover:bg-teal-50 dark:hover:bg-teal-900/30 rounded-lg transition-colors">✏️</button>
                <button onclick={(e) => { e.stopPropagation(); person.id !== undefined && deletePerson(person.id); }} class="p-1.5 text-slate-400 hover:text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-900/30 rounded-lg transition-colors">🗑️</button>
             </div>
             
             <div class="flex items-start space-x-4">
               <div class="w-12 h-12 rounded-full bg-gradient-to-br from-teal-400 to-teal-600 flex items-center justify-center text-white font-bold text-xl shadow-sm">
                 {(person.vorname?.[0] || '')}{(person.name[0] || '')}
               </div>
               <div>
                 <h4 class="font-bold text-slate-800 dark:text-slate-100 text-lg">{person.vorname || ''} {person.name}</h4>
                 <div class="text-sm font-medium text-teal-600 dark:text-teal-400 flex items-center mt-0.5">
                   <span class="mr-1">🏢</span> {getFirmaName(person.firma_id)}
                 </div>
               </div>
             </div>
             
             <div class="mt-5 space-y-2 text-sm text-slate-600 dark:text-slate-400">
               {#if person.mail}
                 <div class="flex items-center"><span class="w-5 text-slate-400">✉️</span> {person.mail}</div>
               {/if}
               {#if person.tel_direkt}
                 <div class="flex items-center"><span class="w-5 text-slate-400">📞</span> {person.tel_direkt} (Dir)</div>
               {/if}
               {#if person.tel_mobil}
                 <div class="flex items-center"><span class="w-5 text-slate-400">📱</span> {person.tel_mobil} (Mobil)</div>
               {/if}
             </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

{#if showDetailModal}
  <div class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      
      <!-- Detail Header -->
      <div class="px-8 py-6 border-b border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-between items-start">
        <div class="flex items-start space-x-4">
           <div class="w-14 h-14 rounded-full bg-gradient-to-br from-teal-400 to-teal-600 flex items-center justify-center text-white font-bold text-2xl shadow-sm">
             {(currentPerson.vorname?.[0] || '')}{(currentPerson.name?.[0] || '')}
           </div>
           <div>
             <h3 class="text-2xl font-bold text-slate-800 dark:text-slate-100">
               {currentPerson.anrede ? currentPerson.anrede + ' ' : ''}{currentPerson.vorname||''} {currentPerson.name}
             </h3>
             {#if currentPerson.firma_id}
               <div class="mt-1 flex items-center bg-teal-50 dark:bg-teal-900/30 text-teal-700 dark:text-teal-300 px-3 py-1 rounded-lg font-medium text-sm w-fit border border-teal-100 dark:border-teal-800">
                 🏢 {getFirmaName(currentPerson.firma_id)}
               </div>
             {/if}
           </div>
        </div>
        
        <div class="flex items-center space-x-2">
          <button onclick={() => openEdit(currentPerson as Ansprechsperson)} class="px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
            <span class="mr-2">✏️</span> Bearbeiten
          </button>
          <button onclick={() => currentPerson.id !== undefined && deletePerson(currentPerson.id)} class="px-4 py-2 bg-rose-50 dark:bg-rose-900/20 text-rose-600 dark:text-rose-400 hover:bg-rose-100 dark:hover:bg-rose-900/40 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
            🗑️ Löschen
          </button>
          <button onclick={() => showDetailModal = false} class="ml-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition">✕</button>
        </div>
      </div>

      <!-- Detail Body -->
      <div class="p-8 overflow-y-auto flex-1 custom-scrollbar space-y-8 bg-slate-50/30 dark:bg-[#0f172a]/30">
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <!-- Info Card -->
          <div class="bg-white dark:bg-slate-800 rounded-xl p-5 border border-slate-200 dark:border-slate-700 shadow-sm">
             <h4 class="font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center"><span class="text-teal-500 mr-2">ℹ️</span> Kontaktinformationen</h4>
             <div class="space-y-3 text-sm">
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">E-Mail:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentPerson.mail || '-'}</span>
               </div>
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">Tel. Direkt:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentPerson.tel_direkt || '-'}</span>
               </div>
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">Tel. Mobil:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentPerson.tel_mobil || '-'}</span>
               </div>
             </div>
          </div>

          <!-- Socials Card -->
          <div class="bg-white dark:bg-slate-800 rounded-xl p-5 border border-slate-200 dark:border-slate-700 shadow-sm">
             <h4 class="font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center"><span class="text-teal-500 mr-2">🔗</span> Social Media</h4>
             <div class="space-y-3 text-sm">
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">LinkedIn:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentPerson.linkedin}
                     <a href={currentPerson.linkedin.startsWith('http') ? currentPerson.linkedin : `https://${currentPerson.linkedin}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentPerson.linkedin}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">Xing:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentPerson.xing}
                     <a href={currentPerson.xing.startsWith('http') ? currentPerson.xing : `https://${currentPerson.xing}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentPerson.xing}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">Facebook:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentPerson.facebook}
                     <a href={currentPerson.facebook.startsWith('http') ? currentPerson.facebook : `https://${currentPerson.facebook}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentPerson.facebook}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
             </div>
          </div>
        </div>

        <!-- Kontakte Timeline for this person -->
        <div>
          <div class="flex items-center justify-between mb-4">
            <h4 class="font-bold text-lg text-slate-800 dark:text-slate-100 flex items-center">💬 Kontakte & Notizen ({detailKontakte.length})</h4>
          </div>
          {#if detailKontakte.length === 0}
            <div class="bg-white dark:bg-slate-800 border border-dashed border-slate-300 dark:border-slate-600 rounded-xl p-6 text-center text-slate-500 text-sm">
              Keine Interaktionen mit dieser Person hinterlegt.
            </div>
          {:else}
            <div class="space-y-4">
              {#each detailKontakte as kontakt}
                <div class="bg-white dark:bg-slate-800 rounded-xl p-5 border border-slate-200 dark:border-slate-700 shadow-sm relative overflow-hidden">
                  <div class="absolute left-0 top-0 bottom-0 w-1 bg-amber-500"></div>
                  <div class="flex justify-between items-start mb-2">
                    <h5 class="font-bold text-slate-800 dark:text-slate-100">{kontakt.betreff}</h5>
                    <div class="text-xs text-slate-500 bg-slate-50 dark:bg-slate-900 border border-slate-100 dark:border-slate-700 px-2 py-1 rounded-md whitespace-nowrap ml-2 font-medium">
                      {kontakt.datum || 'Sometime'} {#if kontakt.von_zeit}({kontakt.von_zeit}){/if}
                    </div>
                  </div>
                  <div class="mb-3 text-xs inline-flex items-center font-semibold px-2 py-0.5 rounded bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-300">
                    {kontakt.typ || 'Unbekannt'}
                  </div>
                  {#if kontakt.text}
                    <div class="text-sm text-slate-600 dark:text-slate-300 bg-slate-50/50 dark:bg-slate-900/30 p-3 rounded-lg border border-slate-100 dark:border-slate-800 whitespace-pre-wrap">
                      {kontakt.text}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>

      </div>
    </div>
  </div>
{/if}

{#if showModal}
  <div class="fixed inset-0 bg-slate-900/40 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700 flex justify-between items-center bg-slate-50/50 dark:bg-slate-900/20">
        <h3 class="text-xl font-bold text-slate-800 dark:text-slate-100">
          {isEditing ? 'Person bearbeiten' : 'Neue Person'}
        </h3>
        <button onclick={() => showModal = false} class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-1">✕</button>
      </div>
      
      <div class="p-6 overflow-y-auto flex-1 custom-scrollbar space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="col-span-1">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Anrede</label>
            <input type="text" bind:value={currentPerson.anrede} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" placeholder="Herr/Frau" />
          </div>
          <div class="col-span-1">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Vorname</label>
            <input type="text" bind:value={currentPerson.vorname} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div class="col-span-1">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Nachname *</label>
            <input type="text" bind:value={currentPerson.name} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Arbeitgeber (Firma)</label>
          <select bind:value={currentPerson.firma_id} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white">
            <option value={undefined}>Keine Firma ausgewählt</option>
            {#each firmen as firma}
              <option value={firma.id}>{firma.name}</option>
            {/each}
          </select>
        </div>

        <hr class="border-slate-100 dark:border-slate-700 my-4" />

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">E-Mail</label>
            <input type="email" bind:value={currentPerson.mail} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div></div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Tel Direkt</label>
            <input type="text" bind:value={currentPerson.tel_direkt} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Tel Mobil</label>
            <input type="text" bind:value={currentPerson.tel_mobil} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>

        <hr class="border-slate-100 dark:border-slate-700 my-4" />
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
           <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">LinkedIn</label>
            <input type="text" bind:value={currentPerson.linkedin} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Xing</label>
            <input type="text" bind:value={currentPerson.xing} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Facebook</label>
            <input type="text" bind:value={currentPerson.facebook} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
        </div>
      </div>
      
      <div class="p-6 border-t border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-end space-x-3">
        <button onclick={() => showModal = false} class="px-5 py-2.5 text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors font-medium">Abbrechen</button>
        <button onclick={savePerson} class="bg-teal-600 hover:bg-teal-700 text-white px-6 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all font-medium">Speichern</button>
      </div>
    </div>
  </div>
{/if}
