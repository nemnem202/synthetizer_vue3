<template>
  <div class="synth-presentation-page">
    <h1>Projet : Synthétiseur Rust / WebAssembly Ultra-Performance</h1>
    
    <p>
      Découvrez mon projet de <strong>synthétiseur</strong> développé en <strong>Rust</strong> et compilé en <strong>WebAssembly</strong> pour des performances ultra-hautes et un contrôle <em>low-level</em>. Un projet unique : rien de comparable n'a vraiment été fait auparavant !
    </p>
    
    <p>
      Pour jouer, vous avez deux options :
      <ul>
        <li>Connecter un contrôleur <strong>MIDI</strong>.</li>
        <li>Utiliser les touches de votre clavier : 
          <code>q, z, s, e, d, f, t, g, y, h, u, j</code> <em>(dans l'ordre des demi-tons)</em>
        </li>
      </ul>
    </p>
    
    <p>
      Pour commencer, cliquez sur le bouton ci-dessous afin de créer vos premières sources sonores.
    </p>
    
    <p>
      Ce synthétiseur sera intégré dans ma prochaine application d'apprentissage de musique en ligne, offrant une expérience interactive et réactive aux utilisateurs.
    </p>
    
    <p>
      Pour découvrir le projet et suivre son évolution, rendez-vous sur 
      <a href="https://github.com/nemnem202" target="_blank">mon GitHub</a>.
    </p>

    <NuxtLink to="/synthpage" class="start-button">Commencer</NuxtLink>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({});
</script>

<style scoped>
.synth-presentation-page {
  max-width: 800px;
  margin: 2rem auto;
  padding: 2rem;
  background-color: rgb(36, 33, 48);
  color: #fff;
  border-radius: 12px;
  box-shadow: 0 8px 20px rgba(0,0,0,0.4);
  font-family: "Roboto", sans-serif;
  line-height: 1.6;
}

.synth-presentation-page h1 {
  color: #fe621b;
  margin-bottom: 1rem;
  font-size: 2rem;
  text-align: center;
}

.synth-presentation-page a {
  color: #fe621b;
  text-decoration: underline;
}

.start-button {
  display: block;
  margin: 2rem auto 0;
  padding: 0.8rem 2rem;
  font-size: 1.2rem;
  background-color: #fe621b;
  border: none;
  border-radius: 8px;
  color: #000;
  text-align: center;
  text-decoration: none;
  transition: all 0.2s ease;
}

.start-button:hover {
  background-color: #fff;
  color: #fe621b;
  transform: scale(1.05);
}
</style>
