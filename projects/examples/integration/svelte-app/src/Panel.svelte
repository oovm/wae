<script lang="ts">
import { getWaeContext } from "@wae/adapter-svelte";

const wae = getWaeContext();
let status = $state("等待 ping…");
let ticks = $state(0);

function ping() {
    ticks += 1;
    status = `client ok · baseUrl=${wae.server ? "set" : "?"} · #${ticks}`;
}
</script>

<section class="stage" aria-live="polite">
  <p class="status">{status}</p>
  <button type="button" class="cta" onclick={ping}>ping client</button>
</section>

<style>
  .stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.1rem;
    padding: 1.35rem 1.6rem;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 1rem;
    backdrop-filter: blur(10px);
  }

  .status {
    margin: 0;
    font-family: var(--mono);
    font-size: 0.82rem;
    color: var(--ink);
    opacity: 0.9;
  }

  .cta {
    appearance: none;
    border: none;
    cursor: pointer;
    font-family: var(--font);
    font-weight: 600;
    font-size: 0.95rem;
    padding: 0.7rem 1.4rem;
    border-radius: 999px;
    color: #1a0800;
    background: linear-gradient(135deg, var(--accent), var(--accent-dim));
    transition:
      transform 0.18s ease,
      filter 0.18s ease;
  }

  .cta:hover {
    transform: translateY(-2px);
    filter: brightness(1.08);
    animation: pulse-ring 1.2s ease-out;
  }

  .cta:active {
    transform: translateY(0);
  }
</style>
