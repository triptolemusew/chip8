import { useContext } from 'react';
import { WASMContext } from '../context/WASM';

export const WASMMain = () => {
    const ctx = useContext(WASMContext)

    if (!ctx.wasm) {
        return <>...</>
    }
    return <>From wasm: {ctx.wasm.run()}</>
}
