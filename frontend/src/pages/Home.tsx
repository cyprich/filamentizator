export default function Home() {
    return (
        <main>
            <h1 className={"mb-12"}>Welcome to Filamentizator!</h1>
            <div className={"flex flex-col gap-16"}>
                <Section name={"Filaments"}/>
                <Section name={"Vendors"}/>
                <Section name={"Materials"}/>
                <Section name={"Account"}/>
            </div>
        </main>
    )
}

type SectionProps = {
    name: string;
}

function Section(props: SectionProps) {
    const dummy = [1, 2, 3, 4];

    return (
        <div className={"flex flex-col gap-4"}>
            <h2>{props.name}</h2>
            <div className={"flex gap-8"}>
                {
                    dummy.map(() => (
                        <div className={"bg-neutral-200 w-48 h-48 flex items-center justify-center"}>
                            Placeholder
                        </div>
                    ))
                }
            </div>
        </div>
    )
}