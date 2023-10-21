const HtmlBlock = ({ html }: { html: string }) => (
  <div
    dangerouslySetInnerHTML={{
      __html: html,
    }}
  />
);

export default HtmlBlock;
