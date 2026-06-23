import StatsBar from '../components/StatsBar';

export default function Home() {
  return (
    <main style={{ padding: '2rem', fontFamily: 'sans-serif' }}>
      <h1 style={{ marginBottom: '0.25rem' }}>YieldLadder</h1>
      <p style={{ marginTop: 0, opacity: 0.6 }}>
        Time-locked USDC vaults on Soroban with auto-routed AMM yield.
      </p>
      <StatsBar />
      <p style={{ marginTop: '2rem', opacity: 0.5, fontSize: '0.85rem' }}>
        Dashboard coming soon.
      </p>
    </main>
  );
}
