<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Kontakt, Firma, Ansprechsperson } from '$lib/types';
  import { onMount } from 'svelte';

  let kontakte = $state<Kontakt[]>([]);
  let firmen = $state<Firma[]>([]);
  let personen = $state<Ansprechsperson[]>([]);
  
  let showModal = $state(false);
  let showDetailModal = $state(false);
  let isEditing = $state(false);
  let currentKontakt = $state<Partial<Kontakt>>({});
  let loading = $state(true);

  async function loadData() {
    loading = true;
    try {
      const [kRes, fRes, pRes] = await Promise.all([
        invoke<Kontakt[]>('get_kontakte'),
        invoke<Firma[]>('get_firmen'),
        invoke<Ansprechsperson[]>('get_ansprechspersonen')
      ]);
      // Sort kontakte by datum descending
      kontakte = kRes.sort((a, b) => {
        if (!a.datum) return 1;
        if (!b.datum) return -1;
        return b.datum.localeCompare(a.datum);
      });
      firmen = fRes;
      personen = pRes;
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
    const today = new Date().toISOString().split('T')[0];
    currentKontakt = { 
      typ: 'Mail', 
      datum: today, 
      von_zeit: '', 
      bis_zeit: '', 
      betreff: '', 
      text: '', 
      partizipierende_personen: '',
      firma_id: undefined,
      ansprechsperson_id: undefined
    };
    showModal = true;
  }

  function openEdit(kontakt: Kontakt) {
    isEditing = true;
    currentKontakt = { ...kontakt };
    showModal = true;
    showDetailModal = false;
  }

  function openDetails(kontakt: Kontakt) {
    currentKontakt = { ...kontakt };
    showDetailModal = true;
  }

  async function deleteKontakt(id: number) {
    if (confirm('Kontakt/Notiz wirklich löschen? Diese Aktion ist endgültig.')) {
      try {
        await invoke('delete_kontakt', { id });
        showDetailModal = false;
        await loadData();
      } catch (e) {
        console.error(e);
        alert('Delete failed.');
      }
    }
  }

  async function saveKontakt() {
    if (!currentKontakt.betreff) {
      alert("Betreff ist erforderlich.");
      return;
    }
    
    if (currentKontakt.firma_id) currentKontakt.firma_id = parseInt(currentKontakt.firma_id as any);
    if (currentKontakt.ansprechsperson_id) currentKontakt.ansprechsperson_id = parseInt(currentKontakt.ansprechsperson_id as any);
    
    try {
      if (isEditing) {
        await invoke('update_kontakt', { kontakt: currentKontakt });
      } else {
        await invoke('create_kontakt', { kontakt: currentKontakt });
      }
      showModal = false;
      await loadData();
    } catch (e) {
      console.error(e);
      alert('Save failed: ' + e);
    }
  }

  function getFirmaName(id?: number) {
    if (!id) return '';
    return firmen.find(f => f.id === id)?.name || 'Unbekannte Firma';
  }

  function getPersonName(id?: number) {
    if (!id) return '';
    const p = personen.find(p => p.id === id);
    if (!p) return 'Unbekannte Person';
    return `${p.vorname || ''} ${p.name}`.trim();
  }
  
  function getIconForTyp(typ?: string) {
    if (typ === 'Mail') return '✉️';
    if (typ === 'Telefon') return '📞';
    if (typ === 'Gespräch') return '🤝';
    return '📝';
  }
</script>

<div class="flex flex-col space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-3xl font-bold tracking-tight text-slate-800 dark:text-slate-100">Kontakt-Historie</h2>
      <p class="text-slate-500 dark:text-slate-400 mt-2">Notizen, Mails und Gesprächsprotokolle.</p>
    </div>
    <button 
      onclick={openCreate}
      class="bg-teal-600 hover:bg-teal-700 text-white px-5 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all duration-200 font-medium flex items-center"
    >
      <span class="mr-2 text-lg">+</span> Neuer Eintrag
    </button>
  </div>

  <div class="relative">
    {#if loading}
      <div class="bg-white dark:bg-slate-800 rounded-2xl p-8 border border-slate-200 dark:border-slate-700 text-center text-slate-500">Lade Daten...</div>
    {:else if kontakte.length === 0}
      <div class="bg-white dark:bg-slate-800 rounded-2xl p-12 border border-slate-200 dark:border-slate-700 text-center flex flex-col items-center">
        <div class="h-16 w-16 bg-slate-100 dark:bg-slate-700 rounded-full flex items-center justify-center mb-4 text-3xl">📭</div>
        <h3 class="text-lg font-medium text-slate-800 dark:text-slate-200">Keine Kontakteinträge gefunden</h3>
        <p class="text-slate-500 mt-1">Starten Sie Ihr Kontakt-Logbuch.</p>
      </div>
    {:else}
      <!-- Timeline Line -->
      <div class="absolute left-8 top-4 bottom-4 w-px bg-slate-200 dark:bg-slate-700 z-0 hidden md:block"></div>
      
      <div class="space-y-6 relative z-10">
        {#each kontakte as kontakt (kontakt.id)}
          <div class="flex flex-col md:flex-row md:items-start gap-4 group">
            <!-- Timeline Marker -->
            <div class="hidden md:flex flex-col items-center pt-1 w-16 shrink-0">
               <div class="w-10 h-10 rounded-full bg-white dark:bg-slate-800 border-4 border-slate-100 dark:border-slate-800 shadow-sm flex items-center justify-center text-lg shadow-teal-500/20 z-10 transition-transform group-hover:scale-110">
                 {getIconForTyp(kontakt.typ)}
               </div>
            </div>
            
            <!-- Card -->
            <div 
              class="flex-1 bg-white dark:bg-slate-800 rounded-2xl p-6 shadow-sm border border-slate-200 dark:border-slate-700 hover:shadow-md transition-shadow relative overflow-hidden cursor-pointer"
              onclick={() => openDetails(kontakt)}
            >
               <!-- Accent border -->
               <div class="absolute left-0 top-0 bottom-0 w-1 bg-teal-500"></div>

               <div class="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity flex space-x-1">
                  <button onclick={(e) => { e.stopPropagation(); openEdit(kontakt); }} class="p-1.5 text-slate-400 hover:text-teal-600 hover:bg-teal-50 dark:hover:bg-teal-900/30 rounded-lg transition-colors">✏️</button>
                  <button onclick={(e) => { e.stopPropagation(); kontakt.id !== undefined && deleteKontakt(kontakt.id); }} class="p-1.5 text-slate-400 hover:text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-900/30 rounded-lg transition-colors">🗑️</button>
               </div>

               <div class="flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-3">
                 <h4 class="font-bold text-xl text-slate-800 dark:text-slate-100">{kontakt.betreff}</h4>
                 <div class="flex items-center text-sm font-medium text-slate-500 bg-slate-100 dark:bg-slate-700/50 px-2.5 py-1 rounded-lg w-fit">
                   📅 {kontakt.datum || 'Sometime'}
                   {#if kontakt.von_zeit} 
                     <span class="ml-1.5 font-normal text-slate-400">• {kontakt.von_zeit}</span>
                   {/if}
                 </div>
               </div>

               <div class="flex flex-wrap gap-2 mb-4">
                 <span class="inline-flex items-center text-xs font-semibold px-2.5 py-1 rounded-full bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300">
                   {kontakt.typ || 'Unbekannt'}
                 </span>
                 
                 {#if kontakt.firma_id}
                 <span class="inline-flex items-center text-xs font-medium px-2.5 py-1 rounded-md bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300">
                   🏢 {getFirmaName(kontakt.firma_id)}
                 </span>
                 {/if}
                 
                 {#if kontakt.ansprechsperson_id}
                 <span class="inline-flex items-center text-xs font-medium px-2.5 py-1 rounded-md bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300">
                   👤 {getPersonName(kontakt.ansprechsperson_id)}
                 </span>
                 {/if}
               </div>

               {#if kontakt.text}
                 <div class="mt-4 p-4 rounded-xl bg-slate-50 dark:bg-slate-900/50 text-slate-600 dark:text-slate-300 text-sm leading-relaxed whitespace-pre-wrap">
                   {kontakt.text}
                 </div>
               {/if}
               
               {#if kontakt.partizipierende_personen}
                 <div class="mt-4 text-xs font-medium text-slate-500 dark:text-slate-400">
                   👥 Teilnehmer: {kontakt.partizipierende_personen}
                 </div>
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
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      
      <!-- Detail Header -->
      <div class="px-8 py-6 border-b border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-between items-start">
        <div class="flex items-start space-x-4">
           <div class="w-12 h-12 rounded-full bg-slate-100 dark:bg-slate-800 border-2 border-slate-200 dark:border-slate-700 flex items-center justify-center text-2xl shadow-sm">
             {getIconForTyp(currentKontakt.typ)}
           </div>
           <div>
             <h3 class="text-2xl font-bold text-slate-800 dark:text-slate-100 leading-tight">
               {currentKontakt.betreff}
             </h3>
             <div class="text-sm font-medium text-slate-500 dark:text-slate-400 mt-1 flex items-center">
               <span class="bg-slate-200 dark:bg-slate-700 px-2 py-0.5 rounded text-slate-700 dark:text-slate-300 mr-2">{currentKontakt.typ || 'Unbekannt'}</span>
               {currentKontakt.datum || 'Ohne Datum'}
               {#if currentKontakt.von_zeit} 
                 <span class="mx-1">•</span> {currentKontakt.von_zeit} {#if currentKontakt.bis_zeit}- {currentKontakt.bis_zeit}{/if}
               {/if}
             </div>
           </div>
        </div>
        
        <div class="flex items-center space-x-2">
          <button onclick={() => openEdit(currentKontakt as Kontakt)} class="px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
            <span class="mr-2">✏️</span> Bearbeiten
          </button>
          <button onclick={() => currentKontakt.id !== undefined && deleteKontakt(currentKontakt.id)} class="px-4 py-2 bg-rose-50 dark:bg-rose-900/20 text-rose-600 dark:text-rose-400 hover:bg-rose-100 dark:hover:bg-rose-900/40 rounded-xl text-sm font-medium transition-colors flex items-center shadow-sm">
            🗑️ Löschen
          </button>
          <button onclick={() => showDetailModal = false} class="ml-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition">✕</button>
        </div>
      </div>

      <!-- Detail Body -->
      <div class="p-8 overflow-y-auto flex-1 custom-scrollbar space-y-6 bg-slate-50/30 dark:bg-[#0f172a]/30">
        
        <div class="flex flex-col space-y-3">
          {#if currentKontakt.firma_id}
            <div class="flex items-center space-x-3 bg-white dark:bg-slate-800 p-3 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
               <span class="text-2xl">🏢</span>
               <div>
                  <div class="text-xs text-slate-500 font-medium">Zugehörige Firma</div>
                  <div class="font-semibold text-slate-800 dark:text-slate-200">{getFirmaName(currentKontakt.firma_id)}</div>
               </div>
            </div>
          {/if}
          
          {#if currentKontakt.ansprechsperson_id}
            <div class="flex items-center space-x-3 bg-white dark:bg-slate-800 p-3 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
               <span class="text-2xl">👤</span>
               <div>
                  <div class="text-xs text-slate-500 font-medium">Ansprechsperson</div>
                  <div class="font-semibold text-slate-800 dark:text-slate-200">{getPersonName(currentKontakt.ansprechsperson_id)}</div>
               </div>
            </div>
          {/if}
          
          {#if currentKontakt.partizipierende_personen}
            <div class="flex items-center space-x-3 bg-white dark:bg-slate-800 p-3 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm">
               <span class="text-2xl">👥</span>
               <div>
                  <div class="text-xs text-slate-500 font-medium">Partizipierende Personen</div>
                  <div class="font-semibold text-slate-800 dark:text-slate-200">{currentKontakt.partizipierende_personen}</div>
               </div>
            </div>
          {/if}
        </div>

        {#if currentKontakt.text}
          <div>
            <h4 class="font-bold text-lg text-slate-800 dark:text-slate-100 flex items-center mb-3">📝 Notizen / Inhalt</h4>
            <div class="bg-white dark:bg-slate-800 p-5 rounded-2xl border border-slate-200 dark:border-slate-700 shadow-sm text-slate-700 dark:text-slate-300 whitespace-pre-wrap leading-relaxed text-sm">
              {currentKontakt.text}
            </div>
          </div>
        {/if}

      </div>
    </div>
  </div>
{/if}

{#if showModal}
  <div class="fixed inset-0 bg-slate-900/40 backdrop-blur-sm z-50 flex items-center justify-center p-4 overflow-y-auto">
    <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-2xl w-full max-w-3xl my-8 flex flex-col overflow-hidden animate-in zoom-in-95 duration-200">
      <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700 flex justify-between items-center bg-slate-50/50 dark:bg-slate-900/20">
        <h3 class="text-xl font-bold text-slate-800 dark:text-slate-100">
          {isEditing ? 'Eintrag bearbeiten' : 'Neuer Eintrag'}
        </h3>
        <button onclick={() => showModal = false} class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-1">✕</button>
      </div>
      
      <div class="p-6 space-y-5">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div class="md:col-span-1">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Typ</label>
            <select bind:value={currentKontakt.typ} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white">
              <option>Mail</option>
              <option>Telefon</option>
              <option>Gespräch</option>
              <option>Sonstiges</option>
            </select>
          </div>
          <div class="md:col-span-3">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Betreff *</label>
            <input type="text" bind:value={currentKontakt.betreff} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Firma (Bezug)</label>
            <select bind:value={currentKontakt.firma_id} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white">
              <option value={undefined}>Keine</option>
              {#each firmen as firma}
                <option value={firma.id}>{firma.name}</option>
              {/each}
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Ansprechsperson</label>
            <select bind:value={currentKontakt.ansprechsperson_id} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white">
              <option value={undefined}>Keine</option>
              {#each personen as p}
                {#if !currentKontakt.firma_id || p.firma_id == currentKontakt.firma_id}
                  <option value={p.id}>{p.vorname || ''} {p.name} {p.firma_id ? `(${getFirmaName(p.firma_id)})` : ''}</option>
                {/if}
              {/each}
            </select>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Datum</label>
            <input type="date" bind:value={currentKontakt.datum} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Von (Zeit)</label>
            <input type="time" bind:value={currentKontakt.von_zeit} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Bis (Zeit)</label>
            <input type="time" bind:value={currentKontakt.bis_zeit} class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Beteiligte Personen</label>
          <input type="text" bind:value={currentKontakt.partizipierende_personen} placeholder="z.B. Herr Meier, Frau Schmidt..." class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-2.5 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white text-sm" />
        </div>

        <div>
           <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Notizen / Text</label>
           <textarea bind:value={currentKontakt.text} rows="6" class="w-full bg-slate-50 dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-teal-500 outline-none dark:text-white custom-scrollbar"></textarea>
        </div>
      </div>
      
      <div class="p-6 border-t border-slate-100 dark:border-slate-700 bg-slate-50/50 dark:bg-slate-900/20 flex justify-end space-x-3 mt-auto">
        <button onclick={() => showModal = false} class="px-5 py-2.5 text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors font-medium">Abbrechen</button>
        <button onclick={saveKontakt} class="bg-teal-600 hover:bg-teal-700 text-white px-6 py-2.5 rounded-xl shadow-md hover:shadow-lg transition-all font-medium">Speichern</button>
      </div>
    </div>
  </div>
{/if}
