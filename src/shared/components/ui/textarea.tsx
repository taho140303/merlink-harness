import { forwardRef, useId, type ReactNode, type TextareaHTMLAttributes } from 'react';
import { cn } from '../../utils/cn';
import { controlClass, Field } from './field';

export interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: ReactNode;
  hint?: ReactNode;
  error?: ReactNode;
  fieldClassName?: string;
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(function Textarea(
  { id, label, hint, error, className, fieldClassName, rows = 3, ...props },
  ref,
) {
  const autoId = useId();
  const textareaId = id ?? autoId;
  const describedBy = hint || error ? `${textareaId}-desc` : undefined;

  return (
    <Field
      id={textareaId}
      label={label}
      hint={hint}
      error={error}
      required={props.required}
      className={fieldClassName}
    >
      <textarea
        ref={ref}
        id={textareaId}
        rows={rows}
        aria-invalid={error ? true : undefined}
        aria-describedby={describedBy}
        className={cn(controlClass, 'resize-y py-2 leading-relaxed', error && 'border-rose-500', className)}
        {...props}
      />
    </Field>
  );
});
