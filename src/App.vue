<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type DashboardData = {
  repoPath: string;
  repoRoot: string | null;
  jjVersion: string | null;
  status: string;
  log: string;
  bookmarks: string;
  suggestions: string[];
  isRepo: boolean;
  jjAvailable: boolean;
  error: string | null;
};

const repoPath = ref("");
const loading = ref(false);
const failure = ref<string | null>(null);
const dashboard = ref<DashboardData | null>(null);

const heroMessage = computed(() => {
  if (loading.value) {
    return "Refreshing your repository pulse...";
  }

  if (failure.value) {
    return "Sutemi couldn't reach jj yet, but the shell is ready.";
  }

  if (!dashboard.value) {
    return "A desktop cockpit for Jujutsu, tuned for status, history, and quick orientation.";
  }

  if (!dashboard.value.jjAvailable) {
    return "Install jj locally and this workspace becomes live immediately.";
  }

  if (!dashboard.value.isRepo) {
    return "Point Sutemi at any Jujutsu repo and it will unfold the current state.";
  }

  return `Tracking ${dashboard.value.repoRoot} with a native jj bridge.`;
});

async function refresh() {
  loading.value = true;
  failure.value = null;

  try {
    dashboard.value = await invoke<DashboardData>("load_dashboard", {
      repoPath: repoPath.value.trim() || null,
    });
  } catch (error) {
    failure.value = error instanceof Error ? error.message : String(error);
  } finally {
    loading.value = false;
  }
}

function applySuggestion(value: string) {
  repoPath.value = value;
  void refresh();
}

onMounted(() => {
  void refresh();
});
</script>

<template>
  <main class="shell">
    <section class="hero surface">
      <div class="hero-copy">
        <p class="eyebrow">Sutemi / JJ Desktop</p>
        <h1>Material-expressive Jujutsu control, without leaving your native desktop.</h1>
        <p class="lede">
          {{ heroMessage }}
        </p>
      </div>

      <div class="hero-actions">
        <label class="field">
          <span>Repository path</span>
          <input
            v-model="repoPath"
            type="text"
            placeholder="Leave blank to inspect the current directory"
            @keydown.enter.prevent="refresh"
          />
        </label>

        <div class="action-row">
          <button class="primary" :disabled="loading" @click="refresh">
            {{ loading ? "Refreshing..." : "Refresh dashboard" }}
          </button>
          <button class="secondary" :disabled="loading" @click="repoPath = ''">
            Use current directory
          </button>
        </div>
      </div>
    </section>

    <section class="metrics">
      <article class="metric surface">
        <span class="metric-label">JJ availability</span>
        <strong>{{ dashboard?.jjAvailable ? "Ready" : "Missing" }}</strong>
      </article>
      <article class="metric surface">
        <span class="metric-label">Repository</span>
        <strong>{{ dashboard?.isRepo ? "Detected" : "Not found" }}</strong>
      </article>
      <article class="metric surface">
        <span class="metric-label">Version</span>
        <strong>{{ dashboard?.jjVersion ?? "Unavailable" }}</strong>
      </article>
    </section>

    <p v-if="failure" class="banner error">{{ failure }}</p>
    <p v-else-if="dashboard?.error" class="banner warning">{{ dashboard.error }}</p>

    <section class="grid">
      <article class="panel surface panel-wide">
        <header>
          <p class="eyebrow">Status</p>
          <h2>Working copy</h2>
        </header>
        <pre>{{ dashboard?.status ?? "Run a refresh to load jj status." }}</pre>
      </article>

      <article class="panel surface">
        <header>
          <p class="eyebrow">History</p>
          <h2>Recent changes</h2>
        </header>
        <pre>{{ dashboard?.log ?? "No history loaded yet." }}</pre>
      </article>

      <article class="panel surface">
        <header>
          <p class="eyebrow">Bookmarks</p>
          <h2>Named positions</h2>
        </header>
        <pre>{{ dashboard?.bookmarks ?? "No bookmarks loaded yet." }}</pre>
      </article>

      <article class="panel surface panel-wide">
        <header>
          <p class="eyebrow">Quick starts</p>
          <h2>Repository suggestions</h2>
        </header>
        <div class="chips">
          <button
            v-for="suggestion in dashboard?.suggestions ?? []"
            :key="suggestion"
            class="chip"
            @click="applySuggestion(suggestion)"
          >
            {{ suggestion }}
          </button>
        </div>
        <p class="footnote">
          This first pass intentionally keeps the command surface narrow: fast status, recent log,
          and bookmarks through a native Tauri bridge.
        </p>
      </article>
    </section>
  </main>
</template>
