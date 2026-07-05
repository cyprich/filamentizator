export type GradientColorIndicatorProps = {
    colors: string[];
    height: "sm" | "lg";
    className?: string
}

export default function GradientColorIndicator(props: GradientColorIndicatorProps) {
    return (
        <div
            className={`h-2 w-full rounded-full bg-linear-to-r shadow ${props.className}`}
            style={{backgroundImage: `linear-gradient(to right,  ${props.colors.join(", ")})`}}
        />
    )
}