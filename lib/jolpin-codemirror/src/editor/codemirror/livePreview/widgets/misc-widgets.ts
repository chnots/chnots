import { type EditorView, WidgetType } from '@codemirror/view';

export class HorizontalRuleWidget extends WidgetType {
  constructor() {
    super();
  }

  eq(_other: HorizontalRuleWidget) {
    return true;
  }

  toDOM(_view: EditorView) {
    const hr = document.createElement('hr');
    hr.className = 'cm-livePreview-hr';
    hr.setAttribute('aria-hidden', 'true');
    return hr;
  }

  ignoreEvent() {
    return true;
  }
}

export class CheckboxWidget extends WidgetType {
  private readonly checked: boolean;

  constructor(checked: boolean) {
    super();
    this.checked = checked;
  }

  eq(other: CheckboxWidget) {
    return other.checked === this.checked;
  }

  toDOM() {
    const input = document.createElement('input');
    input.type = 'checkbox';
    input.checked = this.checked;
    input.className = 'cm-livePreview-checkbox';
    input.setAttribute('aria-hidden', 'true');
    input.disabled = true;
    return input;
  }

  ignoreEvent() {
    return false;
  }
}
