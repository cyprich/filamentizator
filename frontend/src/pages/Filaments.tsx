import {Card, toast, Typography} from "@heroui/react";
import {useEffect, useState} from "react";
import type {Filament} from "../types/filament.ts";
import axios from "axios";
import {BASE_URL} from "../main.tsx";
import BigFilamentCard from "../components/BigFilamentCard.tsx";

export default function Filaments() {
    const [filaments, setFilaments] = useState<Filament[]>([])

    useEffect(() => {
        axios.get<Filament[]>(`${BASE_URL}/filament`)
            .then(resp => setFilaments(resp.data))
            .catch(e => {
                console.error(e)
                toast.danger("Error while getting Filaments", {
                    description: `${e}`
                })
            })
    }, []);

    return (
        <main>
            <Typography type={"h2"}>Filaments</Typography>
            <div className={"grid grid-cols-3 gap-8 my-4"}>
                {
                    filaments.map(f => (
                        <BigFilamentCard filament={f} />
                    ))
                }
            </div>
        </main>
    )
}