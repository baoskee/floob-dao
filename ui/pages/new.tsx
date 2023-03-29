import { PageView } from ".";

// Need to have title
// Description
// And body text with no rich text
// And a submit button
// - That compiles everything and send a proposal to the DAO
const NewStory = () => {
  return (
    <PageView>
      <div className="max-w-lg flex flex-col justify-start items-start"> 
        {/* This keeps component from resizing. */}
        <div className="transparent w-[1000px]"/>
        <div contentEditable="true" className="text-xl">
          Title
        </div>
        <div contentEditable="true" className="text-secondary">
          Description here.
        </div>
        <div contentEditable="true" className="py-4">
          {`Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Morbi
          tincidunt ornare massa eget egestas purus viverra accumsan in. Leo
          integer malesuada nunc vel risus. Id porta nibh venenatis cras sed
          felis. Sed risus pretium quam vulputate dignissim. Viverra maecenas
          accumsan lacus vel facilisis volutpat est velit. Viverra mauris in
          aliquam sem fringilla. Vitae justo eget magna fermentum iaculis eu.
          Egestas diam in arcu cursus euismod quis viverra..`}
        </div>

        <div>
          <button className="bg-primary text-black px-4 py-2 rounded-md hover:text-white hover:bg-black"
          >
            Submit
          </button>
        </div>
      </div>
    </PageView>
  );
};

export default NewStory;
