{{- define "heading" -}}
  {{- $hash := strings.Repeat (conv.ToInt (math.Add .headings_level .level)) "#" -}}
  {{ printf "%s" $hash }}
{{- end -}}
