import type {Filament} from "../types/filament.ts";
import {Card} from "@heroui/react";
import {Link} from "react-router";

export type BigFilamentCardProps = {
    filament: Filament;
}

export default function BigFilamentCard(props: BigFilamentCardProps) {
    return (
        <Link to={`/filaments/${props.filament.id}`}>
            <Card className={"clickable-no-scale"}>
                <Card.Header>
                    <Card.Title>{props.filament.name}</Card.Title>
                </Card.Header>
            </Card>
        </Link>
    )
}