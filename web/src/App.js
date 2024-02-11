import React, { useEffect, useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import MainScreen from './components/Screen';
import * as wasm from '../../pkg';

import './app.css';

import '@fortawesome/fontawesome-svg-core/styles.css';
import { config, library } from '@fortawesome/fontawesome-svg-core';
import {
  faStar,
  faArrowUp,
  faArrowRight,
  faArrowDown,
  faArrowLeft,
  faTrophy,
} from '@fortawesome/free-solid-svg-icons'
import { faGithub } from '@fortawesome/free-brands-svg-icons';

library.add(faStar, faArrowUp, faArrowRight, faArrowDown, faArrowLeft, faTrophy, faGithub);
config.autoAddCss = false

const App = () => {
  const [game, setGame] = useState("TANK");

  useEffect(() => {
    loadRom(game);
  })

  const loadRom = (game) => {
    fetch(`roms/${game}`)
      .then(i => i.arrayBuffer())
      .then(buffer => {
        wasm.run(new Uint8Array(buffer));
      })
  }

  const onSelectGameChange = (e) => {
    setGame(e.target.value);
    loadRom(e.target.value);
  }

  return (
    <div class="bg-main">
      <div class="w-full">
        <main role="main" class="w-full flex h-screen justify-center place-items-center">
          <div class="max-w-[640px] rounded overflow-x-auto shadow-lg">
            <MainScreen />
            <div class="px-6 py-4">
              <div class="flex items-center justify-between ">
                <div class="flex items-center">
                  <div class="relative h-10 w-72 min-w-[200px]">
                    <select onChange={onSelectGameChange} defaultValue={game} class="peer h-full w-full rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 empty:!bg-gray-900 focus:border-2 focus:border-gray-900 focus:border-t-transparent focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50">
                      <option value="TANK">TANK</option>
                      <option value="SPACE_INVADERS">SPACE INVADERS</option>
                      <option value="TETRIS">TETRIS</option>
                    </select>
                    <label
                      class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-900 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-gray-900 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-gray-900 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                      Select a Game
                    </label>
                  </div>
                </div>
                <button
                  class="bg-white hover:bg-gray-100 text-gray-800 py-2 px-2 border border-gray-400 rounded"
                >
                  Start Game
                </button>
              </div>
            </div>
          </div>
        </main>
      </div>
    </div>
  )
}

export default App;
