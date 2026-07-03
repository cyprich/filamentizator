import {Card, Typography} from "@heroui/react";
import type {ReactNode} from "react";

export default function Home() {



    return (
        <main className={"flex flex-col gap-8"}>
            <Section title={"Filaments"} description={"abcd"}>
                <p>blabla</p>
            </Section>
            <div className={"grid grid-cols-2 gap-8"}>
                <Section title={"Vendors"}>
                    Vendors alvlaksjfdj
                </Section>
                <Section title={"Materials"}>
                    asldjfa;lsj
                </Section>
            </div>
            <Section title={"Account"}>
                asldjfa;lsj
            </Section>
        </main>
    )
}

type SectionProps = {
    title: string;
    description?: string;
    children: ReactNode;
    className?: string;
}

function Section(props: SectionProps) {
    return (
        <div>
            <Typography type={"h2"}>{props.title}</Typography>
            {
                props.description && <Typography type={"body-sm"} className={"mb-2"}>
                    { props.description }
                </Typography>
            }
            { props.children }
        </div>
    )
}

