import type { NextPage } from 'next'
import Head from 'next/head'
import { WASMMain } from '../components/WASMMain'

const Home: NextPage = () => {
    return (
        <div>
            <Head>
                <title>NextJS with WebAssembly</title>
                <meta name="description" content="NextJS with WebAssembly" />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main>
                <h1>
                    Welcome
                </h1>

                <div>
                    <canvas id="chip8" height="150" width="150"></canvas>
                    <WASMMain />
                </div>
            </main>
        </div>
    )
}

export default Home
