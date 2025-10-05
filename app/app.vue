<template>
  <NuxtPage />
</template>

<script></script>

<style>
@import url("https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap");

body {
  background-color: rgb(36, 33, 48);
  color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
  font-family: "Roboto", sans-serif;
}

* {
  box-sizing: border-box;
}

*::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

*::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 10px;
}

*::-webkit-scrollbar-thumb {
  background-color: transparent;
  border-radius: 5px;
  border: 1px solid transparent;
}

*::-webkit-scrollbar-thumb:hover {
  background-color: transparent;
}

* {
  scrollbar-width: thin;
  scrollbar-color: #fe621b transparent;
}

*::-webkit-scrollbar-button {
  display: none;
  opacity: 0;
}

button {
  background-color: transparent;
  border: 1px solid #fe621b;
  color: #fe621b;
  svg {
    fill: #fe621b;
  }

  transition: all 0.2s cubic-bezier(0.62, -0.01, 1, 0.4);
}

select {
  border-color: #fe621b;
  background-color: transparent;
  color: #fe621b;
  -webkit-appearance: none; /* Chrome, Safari, Edge */
  -moz-appearance: none; /* Firefox */
  appearance: none; /* Standard */
  background-color: transparent;
  color: #fe621b;
  padding-right: 1.5em;
  transition: all 0.2s cubic-bezier(0.62, -0.01, 1, 0.4);
}

button:hover,
select:hover {
  background-color: #fe621b;
  color: black;

  svg {
    fill: black;
  }
}
</style>
