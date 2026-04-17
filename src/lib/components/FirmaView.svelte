<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Firma, Ansprechsperson, Kontakt } from '$lib/types';
  import { onMount } from 'svelte';

  let firmen = $state<Firma[]>([]);
  let allPersonen = $state<Ansprechsperson[]>([]);
  let allKontakte = $state<Kontakt[]>([]);
  let showModal = $state(false);
  let showDetailModal = $state(false);
  let isEditing = $state(false);
  let currentFirma = $state<Partial<Firma>>({});
  let loading = $state(true);
  let searchQuery = $state('');

  let filteredFirmen = $derived(
    firmen.filter(f => 
       f.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
      (f.ort && f.ort.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (f.mail && f.mail.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );

  let detailPersonen = $derived(allPersonen.filter(p => p.firma_id === currentFirma.id));
  let detailKontakte = $derived(allKontakte.filter(k => k.firma_id === currentFirma.id).sort((a,b) => (b.datum || '').localeCompare(a.datum || '')));

  async function loadData() {
    loading = true;
    try {
      const [fRes, pRes, kRes] = await Promise.all([
        invoke<Firma[]>('get_firmen'),
        invoke<Ansprechsperson[]>('get_ansprechspersonen'),
        invoke<Kontakt[]>('get_kontakte')
      ]);
      firmen = fRes;
      allPersonen = pRes;
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
    currentFirma = { name: '', strasse: '', plz: '', ort: '', land: '', website: '', mail: '', telefon: '', fax: '', facebook: '', instagram: '', x_twitter: '' };
    showModal = true;
  }

  function openEdit(firma: Firma) {
    isEditing = true;
    currentFirma = { ...firma };
    showModal = true;
    showDetailModal = false;
  }

  function openDetails(firma: Firma) {
    currentFirma = { ...firma };
    showDetailModal = true;
  }

  async function deleteFirma(id: number) {
    if (confirm('Firma wirklich löschen? Diese Aktion ist endgültig.')) {
      try {
        await invoke('delete_firma', { id });
        showDetailModal = false;
        await loadData();
      } catch (e) {
        console.error(e);
        alert('Delete failed.');
      }
    }
  }

  async function saveFirma() {
    if (!currentFirma.name) {
      alert("Name ist erforderlich.");
      return;
    }
    try {
      if (isEditing) {
        await invoke('update_firma', { firma: currentFirma });
      } else {
        await invoke('create_firma', { firma: currentFirma });
      }
      showModal = false;
      await loadData();
    } catch (e) {
      console.error(e);
      alert('Save failed: ' + e);
    }
  }
</script>

<div class="flex flex-col space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
  <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
    <div>
      <h2 class="text-3xl font-bold tracking-tight text-slate-800 dark:text-slate-100">Firmen</h2>
      <p class="text-slate-500 dark:text-slate-400 mt-2">Firmen und Geschäftspartner verwalten.</p>
    </div>
    
    <div class="flex w-full md:w-auto items-center space-x-3">
      <div class="relative w-full md:w-64">
        <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-slate-400">🔍</span>
        <input 
          type="text" 
          bind:value={searchQuery}
          placeholder="Firmen suchen..." 
          class="w-full pl-10 pr-4 py-2.5 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white transition-all shadow-sm"
        />
      </div>
      <button 
        onclick={openCreate}
        class="bg-teal-600 hover:bg-teal-700 text-white px-5 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all duration-200 font-medium flex items-center shrink-0"
      >
        <span class="mr-2 text-lg">+</span> Neue Firma
      </button>
    </div>
  </div>

  <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-sm border border-slate-200 dark:border-slate-700 overflow-hidden">
    {#if loading}
      <div class="p-8 text-center text-slate-500">Lade Daten...</div>
    {:else if filteredFirmen.length === 0}
      <div class="p-12 text-center flex flex-col items-center">
        <div class="h-16 w-16 bg-slate-100 dark:bg-slate-700 rounded-full flex items-center justify-center mb-4 text-3xl">📭</div>
        <h3 class="text-lg font-medium text-slate-800 dark:text-slate-200">Keine Firmen gefunden</h3>
        <p class="text-slate-500 mt-1">
          {#if searchQuery}Versuchen Sie einen anderen Suchbegriff.{:else}Fügen Sie Ihre erste Firma hinzu.{/if}
        </p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="bg-slate-50/80 dark:bg-slate-900/50 text-slate-500 dark:text-slate-400 border-b border-slate-200 dark:border-slate-700 text-sm">
              <th class="px-6 py-4 font-medium">Name</th>
              <th class="px-6 py-4 font-medium">Ort</th>
              <th class="px-6 py-4 font-medium">Kontakt</th>
              <th class="px-6 py-4 font-medium text-right">Aktionen</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100 dark:divide-slate-700/50">
            {#each filteredFirmen as firma (firma.id)}
              <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group cursor-pointer" onclick={() => openDetails(firma)}>
                <td class="px-6 py-4">
                  <div class="font-medium text-slate-800 dark:text-slate-200">{firma.name}</div>
                  <div class="text-xs text-slate-500 mt-0.5">{firma.website || '-'}</div>
                </td>
                <td class="px-6 py-4 text-slate-600 dark:text-slate-400">
                  {firma.plz || ''} {firma.ort || '-'} {firma.land ? `(${firma.land})` : ''}
                </td>
                <td class="px-6 py-4 text-slate-600 dark:text-slate-400 text-sm">
                  {#if firma.mail}<div>{firma.mail}</div>{/if}
                  {#if firma.telefon}<div>{firma.telefon}</div>{/if}
                </td>
                <td class="px-6 py-4 text-right">
                  <div class="flex items-center justify-end space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                    <button onclick={(e) => { e.stopPropagation(); openDetails(firma); }} class="px-3 py-1.5 text-xs font-medium text-teal-700 bg-teal-50 hover:bg-teal-100 dark:text-teal-300 dark:bg-teal-900/40 dark:hover:bg-teal-900/60 rounded-lg transition-colors border border-teal-200 dark:border-teal-800">Details</button>
                    <button onclick={(e) => { e.stopPropagation(); openEdit(firma); }} class="p-2 text-slate-400 hover:text-teal-600 hover:bg-teal-50 dark:hover:bg-teal-900/30 rounded-lg transition-colors" title="Bearbeiten">✏️</button>
                    <button onclick={(e) => { e.stopPropagation(); firma.id !== undefined && deleteFirma(firma.id); }} class="p-2 text-slate-400 hover:text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-900/30 rounded-lg transition-colors" title="Löschen">🗑️</button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

{#if showDetailModal}
  <div class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      
      <!-- Detail Header -->
      <div class="px-8 py-6 border-b border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-between items-start">
        <div>
           <div class="flex items-center space-x-3 mb-1">
             <div class="w-10 h-10 rounded-xl bg-teal-100 dark:bg-teal-900/50 flex items-center justify-center text-teal-600 dark:text-teal-400 text-xl font-bold shadow-sm">
               {currentFirma.name?.[0] || '🏢'}
             </div>
             <h3 class="text-2xl font-bold text-slate-800 dark:text-slate-100">{currentFirma.name}</h3>
           </div>
           
           <div class="flex items-center text-sm text-slate-500 dark:text-slate-400 ml-13 space-x-4">
              <span>📍 {currentFirma.plz||''} {currentFirma.ort||'-'}</span>
              {#if currentFirma.website}<span>🌐 <a href={currentFirma.website.startsWith('http') ? currentFirma.website : `https://${currentFirma.website}`} target="_blank" rel="noopener noreferrer" class="hover:underline hover:text-teal-600 dark:hover:text-teal-400">{currentFirma.website}</a></span>{/if}
           </div>
        </div>
        <div class="flex items-center space-x-2">
          <button onclick={() => openEdit(currentFirma as Firma)} class="px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
            <span class="mr-2">✏️</span> Bearbeiten
          </button>
          <button onclick={() => currentFirma.id !== undefined && deleteFirma(currentFirma.id)} class="px-4 py-2 bg-rose-50 dark:bg-rose-900/20 text-rose-600 dark:text-rose-400 hover:bg-rose-100 dark:hover:bg-rose-900/40 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
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
                 <span class="text-slate-500">Adresse:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentFirma.strasse || '-'} <br/>{currentFirma.plz||''} {currentFirma.ort||''} <br/>{currentFirma.land||''}</span>
               </div>
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">E-Mail:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentFirma.mail || '-'}</span>
               </div>
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">Telefon:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentFirma.telefon || '-'}</span>
               </div>
               <div class="grid grid-cols-3 gap-2">
                 <span class="text-slate-500">Fax:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">{currentFirma.fax || '-'}</span>
               </div>
             </div>
          </div>

          <!-- Socials Card -->
          <div class="bg-white dark:bg-slate-800 rounded-xl p-5 border border-slate-200 dark:border-slate-700 shadow-sm">
             <h4 class="font-semibold text-slate-800 dark:text-slate-200 mb-4 flex items-center"><span class="text-teal-500 mr-2">🔗</span> Social Media</h4>
             <div class="space-y-3 text-sm">
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">Website:</span>
                 <span class="col-span-2 text-teal-600 dark:text-teal-400">
                   {#if currentFirma.website}
                     <a href={currentFirma.website.startsWith('http') ? currentFirma.website : `https://${currentFirma.website}`} target="_blank" rel="noopener noreferrer" class="hover:underline">{currentFirma.website}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">Facebook:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentFirma.facebook}
                     <a href={currentFirma.facebook.startsWith('http') ? currentFirma.facebook : `https://${currentFirma.facebook}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentFirma.facebook}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">Instagram:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentFirma.instagram}
                     <a href={currentFirma.instagram.startsWith('http') ? currentFirma.instagram : `https://${currentFirma.instagram}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentFirma.instagram}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
               <div class="grid grid-cols-3 gap-2 items-center">
                 <span class="text-slate-500">X / Twitter:</span>
                 <span class="col-span-2 text-slate-800 dark:text-slate-300">
                   {#if currentFirma.x_twitter}
                     <a href={currentFirma.x_twitter.startsWith('http') ? currentFirma.x_twitter : `https://${currentFirma.x_twitter}`} target="_blank" rel="noopener noreferrer" class="text-teal-600 dark:text-teal-400 hover:underline">{currentFirma.x_twitter}</a>
                   {:else}
                     -
                   {/if}
                 </span>
               </div>
             </div>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <!-- Mitarbeiter -->
          <div>
            <div class="flex items-center justify-between mb-4">
              <h4 class="font-bold text-lg text-slate-800 dark:text-slate-100 flex items-center">👥 Mitarbeiter ({detailPersonen.length})</h4>
            </div>
            {#if detailPersonen.length === 0}
              <div class="bg-white dark:bg-slate-800 border border-dashed border-slate-300 dark:border-slate-600 rounded-xl p-6 text-center text-slate-500 text-sm">
                Keine Mitarbeiter hinterlegt.
              </div>
            {:else}
              <div class="space-y-3">
                {#each detailPersonen as person}
                  <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700 shadow-sm flex items-center space-x-3">
                    <div class="w-10 h-10 rounded-full bg-slate-100 dark:bg-slate-700 flex items-center justify-center text-slate-600 dark:text-slate-300 font-medium text-sm shrink-0">
                      {(person.vorname?.[0]||'')}{(person.name[0]||'')}
                    </div>
                    <div class="min-w-0 flex-1">
                      <div class="font-medium text-slate-800 dark:text-slate-200 truncate">{person.vorname||''} {person.name}</div>
                      <div class="text-xs text-slate-500 truncate">{person.mail || person.tel_direkt || 'Keine Kontaktdaten'}</div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Kontakte -->
          <div>
            <div class="flex items-center justify-between mb-4">
              <h4 class="font-bold text-lg text-slate-800 dark:text-slate-100 flex items-center">💬 Kontakte ({detailKontakte.length})</h4>
            </div>
            {#if detailKontakte.length === 0}
              <div class="bg-white dark:bg-slate-800 border border-dashed border-slate-300 dark:border-slate-600 rounded-xl p-6 text-center text-slate-500 text-sm">
                Keine Interaktionen hinterlegt.
              </div>
            {:else}
              <div class="space-y-3">
                {#each detailKontakte as kontakt}
                  <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700 shadow-sm relative overflow-hidden">
                    <div class="absolute left-0 top-0 bottom-0 w-1 bg-teal-500"></div>
                    <div class="flex justify-between items-start mb-1">
                      <div class="font-medium text-slate-800 dark:text-slate-200 text-sm">{kontakt.betreff}</div>
                      <div class="text-xs text-slate-400 bg-slate-50 dark:bg-slate-900 px-2 py-0.5 rounded-md whitespace-nowrap ml-2">{kontakt.datum || 'Sometime'}</div>
                    </div>
                    <div class="text-xs text-slate-500 line-clamp-2">
                       <span class="font-semibold text-slate-600 dark:text-slate-400">{kontakt.typ || 'Typ'}:</span> {kontakt.text || ''}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>

      </div>
    </div>
  </div>
{/if}

{#if showModal}
  <!-- Modal Backdrop -->
  <div class="fixed inset-0 bg-slate-900/40 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700 flex justify-between items-center bg-slate-50/50 dark:bg-slate-900/20">
        <h3 class="text-xl font-bold text-slate-800 dark:text-slate-100">
          {isEditing ? 'Firma bearbeiten' : 'Neue Firma'}
        </h3>
        <button onclick={() => showModal = false} class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-1">✕</button>
      </div>
      
      <div class="p-6 overflow-y-auto flex-1 custom-scrollbar space-y-4">
        <!-- Basis Daten -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Firmenname *</label>
          <input type="text" bind:value={currentFirma.name} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 focus:border-teal-500 outline-none transition-all dark:text-white" placeholder="Name der Firma" />
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Straße</label>
            <input type="text" bind:value={currentFirma.strasse} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div class="grid grid-cols-3 gap-2">
            <div class="col-span-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">PLZ</label>
              <input type="text" bind:value={currentFirma.plz} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
            </div>
            <div class="col-span-2">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Ort</label>
              <input type="text" bind:value={currentFirma.ort} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Land</label>
            <input type="text" bind:value={currentFirma.land} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Website</label>
            <input type="text" bind:value={currentFirma.website} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>

        <hr class="border-slate-100 dark:border-slate-700 my-4" />

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">E-Mail</label>
            <input type="email" bind:value={currentFirma.mail} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Telefon</label>
            <input type="text" bind:value={currentFirma.telefon} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Fax</label>
            <input type="text" bind:value={currentFirma.fax} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>

        <hr class="border-slate-100 dark:border-slate-700 my-4" />
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
           <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Facebook</label>
            <input type="text" bind:value={currentFirma.facebook} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Instagram</label>
            <input type="text" bind:value={currentFirma.instagram} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">X / Twitter</label>
            <input type="text" bind:value={currentFirma.x_twitter} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none text-sm dark:text-white" />
          </div>
        </div>
      </div>
      
      <div class="p-6 border-t border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-end space-x-3">
        <button onclick={() => showModal = false} class="px-5 py-2.5 text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors font-medium">Abbrechen</button>
        <button onclick={saveFirma} class="bg-teal-600 hover:bg-teal-700 text-white px-6 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all font-medium">Speichern</button>
      </div>
    </div>
  </div>
{/if}
