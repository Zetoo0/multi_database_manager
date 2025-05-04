import { Input } from "./Input";

type InputFieldProps = {
  value: string;
  onValueChange: (value: string) => void;
  label: string;
  unit: string;
};

export const InputField = ({
  value,
  onValueChange,
  label,
  unit,
}: InputFieldProps) => {
  return (
    <div className="flex items-center gap-1 text-xs">
      <label className="mr-8">{label}</label>
      <Input
        className="ml-auto w-16 text-right text-xs leading-none"
        value={value}
        onChange={(e) => onValueChange(e.target.value)}
      />
      <div className="text-muted-foreground">{unit}</div>
    </div>
  );
};
