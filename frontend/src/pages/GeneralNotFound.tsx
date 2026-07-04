import {HeartCrackIcon} from "lucide-react";
import {Typography} from "@heroui/react";
import type {ReactNode} from "react";

export type NotFoundProps = {
    title: string;
    children: ReactNode;
}

export default function GeneralNotFound(props: NotFoundProps) {
    return (
        <main className={"flex flex-col items-center justify-center -mt-12 mb-12 [&_a]:clickable-no-scale [&_a]:underline"}>
            <HeartCrackIcon className={"size-16"}/>
            <Typography type={"h1"} className={"mb-4"}>{props.title}</Typography>
            { props.children }
        </main>
    )
}
